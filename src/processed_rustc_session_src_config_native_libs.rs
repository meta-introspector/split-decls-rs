/* FP:native_libs.rs-0001 */ // Parser for the `-l` command-line option, which links the generated crate to
/* FP:native_libs.rs-0002 */ // a native library.
/* FP:native_libs.rs-0003 */ //
/* FP:native_libs.rs-0004 */ // (There is also a similar but separate syntax for `#[link]` attributes,
/* FP:native_libs.rs-0005 */ // which have their own parser in `rustc_metadata`.)
/* FP:native_libs.rs-0006 */ 
/* FP:native_libs.rs-0007 */ use crate::rustc_feature::UnstableFeatures;
/* FP:native_libs.rs-0008 */ use crate::rustc_complete::attrs::NativeLibKind;
/* FP:native_libs.rs-0009 */ 
/* FP:native_libs.rs-0010 */ use crate::EarlyDiagCtxt;
/* FP:native_libs.rs-0011 */ use crate::config::UnstableOptions;
/* FP:native_libs.rs-0012 */ use crate::utils::NativeLib;
/* FP:native_libs.rs-0013 */ 
/* FP:native_libs.rs-0014 */ #[cfg(test)]
/* FP:native_libs.rs-0016 */ 
/* FP:native_libs.rs-0017 */ /// Parses all `-l` options.
/* FP:native_libs.rs-0018 */ pub(crate) fn parse_native_libs(
/* FP:native_libs.rs-0019 */     early_dcx: &EarlyDiagCtxt,
/* FP:native_libs.rs-0020 */     unstable_opts: &UnstableOptions,
/* FP:native_libs.rs-0021 */     unstable_features: UnstableFeatures,
/* FP:native_libs.rs-0022 */     matches: &getopts::Matches,
/* FP:native_libs.rs-0023 */ ) -> Vec<NativeLib> {
/* FP:native_libs.rs-0024 */     let cx = ParseNativeLibCx {
/* FP:native_libs.rs-0025 */         early_dcx,
/* FP:native_libs.rs-0026 */         unstable_options_enabled: unstable_opts.unstable_options,
/* FP:native_libs.rs-0027 */         is_nightly: unstable_features.is_nightly_build(),
/* FP:native_libs.rs-0028 */     };
/* FP:native_libs.rs-0029 */     matches.opt_strs("l").into_iter().map(|value| parse_native_lib(&cx, &value)).collect()
/* FP:native_libs.rs-0030 */ }
/* FP:native_libs.rs-0031 */ 
/* FP:native_libs.rs-0032 */ struct ParseNativeLibCx<'a> {
/* FP:native_libs.rs-0033 */     early_dcx: &'a EarlyDiagCtxt,
/* FP:native_libs.rs-0034 */     unstable_options_enabled: bool,
/* FP:native_libs.rs-0035 */     is_nightly: bool,
/* FP:native_libs.rs-0036 */ }
/* FP:native_libs.rs-0037 */ 
/* FP:native_libs.rs-0038 */ impl ParseNativeLibCx<'_> {
/* FP:native_libs.rs-0039 */     /// If unstable values are not permitted, exits with a fatal error made by
/* FP:native_libs.rs-0040 */     /// combining the given strings.
/* FP:native_libs.rs-0041 */     fn on_unstable_value(&self, message: &str, if_nightly: &str, if_stable: &str) {
/* FP:native_libs.rs-0042 */         if self.unstable_options_enabled {
/* FP:native_libs.rs-0043 */             return;
/* FP:native_libs.rs-0044 */         }
/* FP:native_libs.rs-0045 */ 
/* FP:native_libs.rs-0046 */         let suffix = if self.is_nightly { if_nightly } else { if_stable };
/* FP:native_libs.rs-0047 */         self.early_dcx.early_fatal(format!("{message}{suffix}"));
/* FP:native_libs.rs-0048 */     }
/* FP:native_libs.rs-0049 */ }
/* FP:native_libs.rs-0050 */ 
/* FP:native_libs.rs-0051 */ /// Parses the value of a single `-l` option.
/* FP:native_libs.rs-0052 */ fn parse_native_lib(cx: &ParseNativeLibCx<'_>, value: &str) -> NativeLib {
/* FP:native_libs.rs-0053 */     let NativeLibParts { kind, modifiers, name, new_name } = split_native_lib_value(value);
/* FP:native_libs.rs-0054 */ 
/* FP:native_libs.rs-0055 */     let kind = kind.map_or(NativeLibKind::Unspecified, |kind| match kind {
/* FP:native_libs.rs-0056 */         "static" => NativeLibKind::Static { bundle: None, whole_archive: None },
/* FP:native_libs.rs-0057 */         "dylib" => NativeLibKind::Dylib { as_needed: None },
/* FP:native_libs.rs-0058 */         "framework" => NativeLibKind::Framework { as_needed: None },
/* FP:native_libs.rs-0059 */         "link-arg" => {
/* FP:native_libs.rs-0060 */             cx.on_unstable_value(
/* FP:native_libs.rs-0061 */                 "library kind `link-arg` is unstable",
/* FP:native_libs.rs-0062 */                 ", the `-Z unstable-options` flag must also be passed to use it",
/* FP:native_libs.rs-0063 */                 " and only accepted on the nightly compiler",
/* FP:native_libs.rs-0064 */             );
/* FP:native_libs.rs-0065 */             NativeLibKind::LinkArg
/* FP:native_libs.rs-0066 */         }
/* FP:native_libs.rs-0067 */         _ => cx.early_dcx.early_fatal(format!(
/* FP:native_libs.rs-0068 */             "unknown library kind `{kind}`, expected one of: static, dylib, framework, link-arg"
/* FP:native_libs.rs-0069 */         )),
/* FP:native_libs.rs-0070 */     });
/* FP:native_libs.rs-0071 */ 
/* FP:native_libs.rs-0072 */     // Provisionally create the result, so that modifiers can modify it.
/* FP:native_libs.rs-0073 */     let mut native_lib = NativeLib {
/* FP:native_libs.rs-0074 */         name: name.to_owned(),
/* FP:native_libs.rs-0075 */         new_name: new_name.map(str::to_owned),
/* FP:native_libs.rs-0076 */         kind,
/* FP:native_libs.rs-0077 */         verbatim: None,
/* FP:native_libs.rs-0078 */     };
/* FP:native_libs.rs-0079 */ 
/* FP:native_libs.rs-0080 */     if let Some(modifiers) = modifiers {
/* FP:native_libs.rs-0081 */         // If multiple modifiers are present, they are separated by commas.
/* FP:native_libs.rs-0082 */         for modifier in modifiers.split(',') {
/* FP:native_libs.rs-0083 */             parse_and_apply_modifier(cx, modifier, &mut native_lib);
/* FP:native_libs.rs-0084 */         }
/* FP:native_libs.rs-0085 */     }
/* FP:native_libs.rs-0086 */ 
/* FP:native_libs.rs-0087 */     if native_lib.name.is_empty() {
/* FP:native_libs.rs-0088 */         cx.early_dcx.early_fatal("library name must not be empty");
/* FP:native_libs.rs-0089 */     }
/* FP:native_libs.rs-0090 */ 
/* FP:native_libs.rs-0091 */     native_lib
/* FP:native_libs.rs-0092 */ }
/* FP:native_libs.rs-0093 */ 
/* FP:native_libs.rs-0094 */ /// Parses one of the comma-separated modifiers (prefixed by `+` or `-`), and
/* FP:native_libs.rs-0095 */ /// modifies `native_lib` appropriately.
/* FP:native_libs.rs-0096 */ ///
/* FP:native_libs.rs-0097 */ /// Exits with a fatal error if a malformed/unknown/inappropriate modifier is
/* FP:native_libs.rs-0098 */ /// found.
/* FP:native_libs.rs-0099 */ fn parse_and_apply_modifier(cx: &ParseNativeLibCx<'_>, modifier: &str, native_lib: &mut NativeLib) {
/* FP:native_libs.rs-0100 */     let early_dcx = cx.early_dcx;
/* FP:native_libs.rs-0101 */ 
/* FP:native_libs.rs-0102 */     // Split off the leading `+` or `-` into a boolean value.
/* FP:native_libs.rs-0103 */     let (modifier, value) = match modifier.split_at_checked(1) {
/* FP:native_libs.rs-0104 */         Some(("+", m)) => (m, true),
/* FP:native_libs.rs-0105 */         Some(("-", m)) => (m, false),
/* FP:native_libs.rs-0106 */         _ => cx.early_dcx.early_fatal(
/* FP:native_libs.rs-0107 */             "invalid linking modifier syntax, expected '+' or '-' prefix \
/* FP:native_libs.rs-0108 */              before one of: bundle, verbatim, whole-archive, as-needed",
/* FP:native_libs.rs-0109 */         ),
/* FP:native_libs.rs-0110 */     };
/* FP:native_libs.rs-0111 */ 
/* FP:native_libs.rs-0112 */     // Assigns the value (from `+` or `-`) to an empty `Option<bool>`, or emits
/* FP:native_libs.rs-0113 */     // a fatal error if the option has already been set.
/* FP:native_libs.rs-0114 */     let assign_modifier = |opt_bool: &mut Option<bool>| {
/* FP:native_libs.rs-0115 */         if opt_bool.is_some() {
/* FP:native_libs.rs-0116 */             let msg = format!("multiple `{modifier}` modifiers in a single `-l` option");
/* FP:native_libs.rs-0117 */             early_dcx.early_fatal(msg)
/* FP:native_libs.rs-0118 */         }
/* FP:native_libs.rs-0119 */         *opt_bool = Some(value);
/* FP:native_libs.rs-0120 */     };
/* FP:native_libs.rs-0121 */ 
/* FP:native_libs.rs-0122 */     // Check that the modifier is applicable to the native lib kind, and apply it.
/* FP:native_libs.rs-0123 */     match (modifier, &mut native_lib.kind) {
/* FP:native_libs.rs-0124 */         ("bundle", NativeLibKind::Static { bundle, .. }) => assign_modifier(bundle),
/* FP:native_libs.rs-0125 */         ("bundle", _) => early_dcx
/* FP:native_libs.rs-0126 */             .early_fatal("linking modifier `bundle` is only compatible with `static` linking kind"),
/* FP:native_libs.rs-0127 */ 
/* FP:native_libs.rs-0128 */         ("verbatim", _) => assign_modifier(&mut native_lib.verbatim),
/* FP:native_libs.rs-0129 */ 
/* FP:native_libs.rs-0130 */         ("whole-archive", NativeLibKind::Static { whole_archive, .. }) => {
/* FP:native_libs.rs-0131 */             assign_modifier(whole_archive)
/* FP:native_libs.rs-0132 */         }
/* FP:native_libs.rs-0133 */         ("whole-archive", _) => early_dcx.early_fatal(
/* FP:native_libs.rs-0134 */             "linking modifier `whole-archive` is only compatible with `static` linking kind",
/* FP:native_libs.rs-0135 */         ),
/* FP:native_libs.rs-0136 */ 
/* FP:native_libs.rs-0137 */         ("as-needed", NativeLibKind::Dylib { as_needed })
/* FP:native_libs.rs-0138 */         | ("as-needed", NativeLibKind::Framework { as_needed }) => {
/* FP:native_libs.rs-0139 */             cx.on_unstable_value(
/* FP:native_libs.rs-0140 */                 "linking modifier `as-needed` is unstable",
/* FP:native_libs.rs-0141 */                 ", the `-Z unstable-options` flag must also be passed to use it",
/* FP:native_libs.rs-0142 */                 " and only accepted on the nightly compiler",
/* FP:native_libs.rs-0143 */             );
/* FP:native_libs.rs-0144 */             assign_modifier(as_needed)
/* FP:native_libs.rs-0145 */         }
/* FP:native_libs.rs-0146 */         ("as-needed", _) => early_dcx.early_fatal(
/* FP:native_libs.rs-0147 */             "linking modifier `as-needed` is only compatible with \
/* FP:native_libs.rs-0148 */              `dylib` and `framework` linking kinds",
/* FP:native_libs.rs-0149 */         ),
/* FP:native_libs.rs-0150 */ 
/* FP:native_libs.rs-0151 */         _ => early_dcx.early_fatal(format!(
/* FP:native_libs.rs-0152 */             "unknown linking modifier `{modifier}`, expected one \
/* FP:native_libs.rs-0153 */              of: bundle, verbatim, whole-archive, as-needed"
/* FP:native_libs.rs-0154 */         )),
/* FP:native_libs.rs-0155 */     }
/* FP:native_libs.rs-0156 */ }
/* FP:native_libs.rs-0157 */ 
/* FP:native_libs.rs-0158 */ #[derive(Debug, PartialEq, Eq)]
/* FP:native_libs.rs-0159 */ struct NativeLibParts<'a> {
/* FP:native_libs.rs-0160 */     kind: Option<&'a str>,
/* FP:native_libs.rs-0161 */     modifiers: Option<&'a str>,
/* FP:native_libs.rs-0162 */     name: &'a str,
/* FP:native_libs.rs-0163 */     new_name: Option<&'a str>,
/* FP:native_libs.rs-0164 */ }
/* FP:native_libs.rs-0165 */ 
/* FP:native_libs.rs-0166 */ /// Splits a string of the form `[KIND[:MODIFIERS]=]NAME[:NEW_NAME]` into those
/* FP:native_libs.rs-0167 */ /// individual parts. This cannot fail, but the resulting strings require
/* FP:native_libs.rs-0168 */ /// further validation.
/* FP:native_libs.rs-0169 */ fn split_native_lib_value(value: &str) -> NativeLibParts<'_> {
/* FP:native_libs.rs-0170 */     // Split the initial value into `[KIND=]NAME`.
/* FP:native_libs.rs-0171 */     let name = value;
/* FP:native_libs.rs-0172 */     let (kind, name) = match name.split_once('=') {
/* FP:native_libs.rs-0173 */         Some((prefix, name)) => (Some(prefix), name),
/* FP:native_libs.rs-0174 */         None => (None, name),
/* FP:native_libs.rs-0175 */     };
/* FP:native_libs.rs-0176 */ 
/* FP:native_libs.rs-0177 */     // Split the kind part, if present, into `KIND[:MODIFIERS]`.
/* FP:native_libs.rs-0178 */     let (kind, modifiers) = match kind {
/* FP:native_libs.rs-0179 */         Some(kind) => match kind.split_once(':') {
/* FP:native_libs.rs-0180 */             Some((kind, modifiers)) => (Some(kind), Some(modifiers)),
/* FP:native_libs.rs-0181 */             None => (Some(kind), None),
/* FP:native_libs.rs-0182 */         },
/* FP:native_libs.rs-0183 */         None => (None, None),
/* FP:native_libs.rs-0184 */     };
/* FP:native_libs.rs-0185 */ 
/* FP:native_libs.rs-0186 */     // Split the name part into `NAME[:NEW_NAME]`.
/* FP:native_libs.rs-0187 */     let (name, new_name) = match name.split_once(':') {
/* FP:native_libs.rs-0188 */         Some((name, new_name)) => (name, Some(new_name)),
/* FP:native_libs.rs-0189 */         None => (name, None),
/* FP:native_libs.rs-0190 */     };
/* FP:native_libs.rs-0191 */ 
/* FP:native_libs.rs-0192 */     NativeLibParts { kind, modifiers, name, new_name }
/* FP:native_libs.rs-0193 */ }