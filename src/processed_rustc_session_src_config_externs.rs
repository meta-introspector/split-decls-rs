/* FP:externs.rs-0001 */ // This module contains code to help parse and manipulate `--extern` arguments.
/* FP:externs.rs-0002 */ 
/* FP:externs.rs-0003 */ use std::path::PathBuf;
/* FP:externs.rs-0004 */ 
/* FP:externs.rs-0005 */ use crate::rustc_complete::{Diag, FatalAbort};
/* FP:externs.rs-0006 */ 
/* FP:externs.rs-0007 */ use super::UnstableOptions;
/* FP:externs.rs-0008 */ use crate::EarlyDiagCtxt;
/* FP:externs.rs-0009 */ 
/* FP:externs.rs-0010 */ #[cfg(test)]
/* FP:externs.rs-0012 */ 
/* FP:externs.rs-0013 */ /// Represents the pieces of an `--extern` argument.
/* FP:externs.rs-0014 */ pub(crate) struct ExternOpt {
/* FP:externs.rs-0015 */     pub(crate) crate_name: String,
/* FP:externs.rs-0016 */     pub(crate) path: Option<PathBuf>,
/* FP:externs.rs-0017 */     pub(crate) options: Option<String>,
/* FP:externs.rs-0018 */ }
/* FP:externs.rs-0019 */ 
/* FP:externs.rs-0020 */ /// Breaks out the major components of an `--extern` argument.
/* FP:externs.rs-0021 */ ///
/* FP:externs.rs-0022 */ /// The options field will be a string containing comma-separated options that will need further
/* FP:externs.rs-0023 */ /// parsing and processing.
/* FP:externs.rs-0024 */ pub(crate) fn split_extern_opt<'a>(
/* FP:externs.rs-0025 */     early_dcx: &'a EarlyDiagCtxt,
/* FP:externs.rs-0026 */     unstable_opts: &UnstableOptions,
/* FP:externs.rs-0027 */     extern_opt: &str,
/* FP:externs.rs-0028 */ ) -> Result<ExternOpt, Diag<'a, FatalAbort>> {
/* FP:externs.rs-0029 */     let (name, path) = match extern_opt.split_once('=') {
/* FP:externs.rs-0030 */         None => (extern_opt.to_string(), None),
/* FP:externs.rs-0031 */         Some((name, path)) => (name.to_string(), Some(PathBuf::from(path))),
/* FP:externs.rs-0032 */     };
/* FP:externs.rs-0033 */     let (options, crate_name) = match name.split_once(':') {
/* FP:externs.rs-0034 */         None => (None, name),
/* FP:externs.rs-0035 */         Some((opts, crate_name)) => {
/* FP:externs.rs-0036 */             if unstable_opts.namespaced_crates && crate_name.starts_with(':') {
/* FP:externs.rs-0037 */                 // If the name starts with `:`, we know this was actually something like `foo::bar` and
/* FP:externs.rs-0038 */                 // not a set of options. We can just use the original name as the crate name.
/* FP:externs.rs-0039 */                 (None, name)
/* FP:externs.rs-0040 */             } else {
/* FP:externs.rs-0041 */                 (Some(opts.to_string()), crate_name.to_string())
/* FP:externs.rs-0042 */             }
/* FP:externs.rs-0043 */         }
/* FP:externs.rs-0044 */     };
/* FP:externs.rs-0045 */ 
/* FP:externs.rs-0046 */     if !valid_crate_name(&crate_name, unstable_opts) {
/* FP:externs.rs-0047 */         let mut error = early_dcx.early_struct_fatal(format!(
/* FP:externs.rs-0048 */             "crate name `{crate_name}` passed to `--extern` is not a valid ASCII identifier"
/* FP:externs.rs-0049 */         ));
/* FP:externs.rs-0050 */         let adjusted_name = crate_name.replace('-', "_");
/* FP:externs.rs-0051 */         if is_ascii_ident(&adjusted_name) {
/* FP:externs.rs-0052 */             #[allow(rustc::diagnostic_outside_of_impl)] // FIXME
/* FP:externs.rs-0053 */             error
/* FP:externs.rs-0054 */                 .help(format!("consider replacing the dashes with underscores: `{adjusted_name}`"));
/* FP:externs.rs-0055 */         }
/* FP:externs.rs-0056 */         return Err(error);
/* FP:externs.rs-0057 */     }
/* FP:externs.rs-0058 */ 
/* FP:externs.rs-0059 */     Ok(ExternOpt { crate_name, path, options })
/* FP:externs.rs-0060 */ }
/* FP:externs.rs-0061 */ 
/* FP:externs.rs-0062 */ fn valid_crate_name(name: &str, unstable_opts: &UnstableOptions) -> bool {
/* FP:externs.rs-0063 */     match name.split_once("::") {
/* FP:externs.rs-0064 */         Some((a, b)) if unstable_opts.namespaced_crates => is_ascii_ident(a) && is_ascii_ident(b),
/* FP:externs.rs-0065 */         Some(_) => false,
/* FP:externs.rs-0066 */         None => is_ascii_ident(name),
/* FP:externs.rs-0067 */     }
/* FP:externs.rs-0068 */ }
/* FP:externs.rs-0069 */ 
/* FP:externs.rs-0070 */ fn is_ascii_ident(string: &str) -> bool {
/* FP:externs.rs-0071 */     let mut chars = string.chars();
/* FP:externs.rs-0072 */     if let Some(start) = chars.next()
/* FP:externs.rs-0073 */         && (start.is_ascii_alphabetic() || start == '_')
/* FP:externs.rs-0074 */     {
/* FP:externs.rs-0075 */         chars.all(|char| char.is_ascii_alphanumeric() || char == '_')
/* FP:externs.rs-0076 */     } else {
/* FP:externs.rs-0077 */         false
/* FP:externs.rs-0078 */     }
/* FP:externs.rs-0079 */ }