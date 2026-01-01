/* FP:lib.rs-0001 */ // Lints, aka compiler warnings.
/* FP:lib.rs-0002 */ //
/* FP:lib.rs-0003 */ // A 'lint' check is a kind of miscellaneous constraint that a user _might_
/* FP:lib.rs-0004 */ // want to enforce, but might reasonably want to permit as well, on a
/* FP:lib.rs-0005 */ // module-by-module basis. They contrast with static constraints enforced by
/* FP:lib.rs-0006 */ // other phases of the compiler, which are generally required to hold in order
/* FP:lib.rs-0007 */ // to compile the program at all.
/* FP:lib.rs-0008 */ //
/* FP:lib.rs-0009 */ // Most lints can be written as [`LintPass`] instances. These run after
/* FP:lib.rs-0010 */ // all other analyses. The `LintPass`es built into rustc are defined
/* FP:lib.rs-0011 */ // within [crate::rustc_session::lint::builtin],
/* FP:lib.rs-0012 */ // which has further comments on how to add such a lint.
/* FP:lib.rs-0013 */ // rustc can also load external lint plugins, as is done for Clippy.
/* FP:lib.rs-0014 */ //
/* FP:lib.rs-0015 */ // See <https://rustc-dev-guide.rust-lang.org/diagnostics.html> for an
/* FP:lib.rs-0016 */ // overview of how lints are implemented.
/* FP:lib.rs-0017 */ //
/* FP:lib.rs-0018 */ // ## Note
/* FP:lib.rs-0019 */ //
/* FP:lib.rs-0020 */ // This API is completely unstable and subject to change.
/* FP:lib.rs-0021 */ 
/* FP:lib.rs-0022 */ // tidy-alphabetical-start
/* FP:lib.rs-0023 */ #[allow(internal_features)]
/* FP:lib.rs-0024 */ #[doc(html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")]
/* FP:lib.rs-0025 */ #[doc(rust_logo)]
/* FP:lib.rs-0026 */ #[feature(array_windows)]
/* FP:lib.rs-0027 */ #[feature(assert_matches)]
/* FP:lib.rs-0028 */ #[feature(box_patterns)]
/* FP:lib.rs-0029 */ #[feature(if_let_guard)]
/* FP:lib.rs-0030 */ #[feature(iter_order_by)]
/* FP:lib.rs-0031 */ #[feature(rustc_attrs)]
/* FP:lib.rs-0032 */ #[feature(rustdoc_internals)]
/* FP:lib.rs-0033 */ #[feature(try_blocks)]
/* FP:lib.rs-0034 */ // tidy-alphabetical-end
/* FP:lib.rs-0035 */ 
/* FP:lib.rs-0084 */ 
/* FP:lib.rs-0085 */ use async_closures::AsyncClosureUsage;
/* FP:lib.rs-0086 */ use async_fn_in_trait::AsyncFnInTrait;
/* FP:lib.rs-0087 */ use autorefs::*;
/* FP:lib.rs-0088 */ use builtin::*;
/* FP:lib.rs-0089 */ use dangling::*;
/* FP:lib.rs-0090 */ use default_could_be_derived::DefaultCouldBeDerived;
/* FP:lib.rs-0091 */ use deref_into_dyn_supertrait::*;
/* FP:lib.rs-0092 */ use drop_forget_useless::*;
/* FP:lib.rs-0093 */ use enum_intrinsics_non_enums::EnumIntrinsicsNonEnums;
/* FP:lib.rs-0094 */ use for_loops_over_fallibles::*;
/* FP:lib.rs-0095 */ use if_let_rescope::IfLetRescope;
/* FP:lib.rs-0096 */ use impl_trait_overcaptures::ImplTraitOvercaptures;
/* FP:lib.rs-0097 */ use internal::*;
/* FP:lib.rs-0098 */ use invalid_from_utf8::*;
/* FP:lib.rs-0099 */ use let_underscore::*;
/* FP:lib.rs-0100 */ use lifetime_syntax::*;
/* FP:lib.rs-0101 */ use macro_expr_fragment_specifier_2024_migration::*;
/* FP:lib.rs-0102 */ use map_unit_fn::*;
/* FP:lib.rs-0103 */ use multiple_supertrait_upcastable::*;
/* FP:lib.rs-0104 */ use non_ascii_idents::*;
/* FP:lib.rs-0105 */ use non_fmt_panic::NonPanicFmt;
/* FP:lib.rs-0106 */ use non_local_def::*;
/* FP:lib.rs-0107 */ use nonstandard_style::*;
/* FP:lib.rs-0108 */ use noop_method_call::*;
/* FP:lib.rs-0109 */ use opaque_hidden_inferred_bound::*;
/* FP:lib.rs-0110 */ use pass_by_value::*;
/* FP:lib.rs-0111 */ use precedence::*;
/* FP:lib.rs-0112 */ use ptr_nulls::*;
/* FP:lib.rs-0113 */ use redundant_semicolon::*;
/* FP:lib.rs-0114 */ use reference_casting::*;
/* FP:lib.rs-0115 */ use crate::rustc_complete::def_id::LocalModDefId;
/* FP:lib.rs-0116 */ use crate::rustc_complete::query::Providers;
/* FP:lib.rs-0117 */ use crate::rustc_complete::ty::TyCtxt;
/* FP:lib.rs-0118 */ use shadowed_into_iter::ShadowedIntoIter;
/* FP:lib.rs-0119 */ pub use shadowed_into_iter::{ARRAY_INTO_ITER, BOXED_SLICE_INTO_ITER};
/* FP:lib.rs-0120 */ use static_mut_refs::*;
/* FP:lib.rs-0121 */ use traits::*;
/* FP:lib.rs-0122 */ use transmute::CheckTransmutes;
/* FP:lib.rs-0123 */ use types::*;
/* FP:lib.rs-0124 */ use unit_bindings::*;
/* FP:lib.rs-0125 */ use unqualified_local_imports::*;
/* FP:lib.rs-0126 */ use unused::*;
/* FP:lib.rs-0127 */ 
/* FP:lib.rs-0128 */ #[rustfmt::skip]
/* FP:lib.rs-0129 */ pub use builtin::{MissingDoc, SoftLints};
/* FP:lib.rs-0130 */ pub use context::{CheckLintNameResult, EarlyContext, LateContext, LintContext, LintStore};
/* FP:lib.rs-0131 */ pub use early::diagnostics::decorate_builtin_lint;
/* FP:lib.rs-0132 */ pub use early::{EarlyCheckNode, check_ast_node};
/* FP:lib.rs-0133 */ pub use late::{check_crate, late_lint_mod, unerased_lint_store};
/* FP:lib.rs-0134 */ pub use levels::LintLevelsBuilder;
/* FP:lib.rs-0135 */ pub use passes::{EarlyLintPass, LateLintPass};
/* FP:lib.rs-0136 */ pub use crate::rustc_complete::BufferedEarlyLint;
/* FP:lib.rs-0137 */ pub use crate::rustc_complete::lint::Level::{self, *};
/* FP:lib.rs-0138 */ pub use crate::rustc_complete::lint::{FutureIncompatibleInfo, Lint, LintId, LintPass, LintVec};
/* FP:lib.rs-0139 */ 
/* FP:lib.rs-0140 */ rustc_fluent_macro::fluent_messages! { "../messages.ftl" }
/* FP:lib.rs-0141 */ 
/* FP:lib.rs-0142 */ pub fn provide(providers: &mut Providers) {
/* FP:lib.rs-0143 */     levels::provide(providers);
/* FP:lib.rs-0144 */     expect::provide(providers);
/* FP:lib.rs-0145 */     foreign_modules::provide(providers);
/* FP:lib.rs-0146 */     *providers = Providers { lint_mod, ..*providers };
/* FP:lib.rs-0147 */ }
/* FP:lib.rs-0148 */ 
/* FP:lib.rs-0149 */ fn lint_mod(tcx: TyCtxt<'_>, module_def_id: LocalModDefId) {
/* FP:lib.rs-0150 */     late_lint_mod(tcx, module_def_id, BuiltinCombinedModuleLateLintPass::new());
/* FP:lib.rs-0151 */ }
/* FP:lib.rs-0152 */ 
/* FP:lib.rs-0153 */ early_lint_methods!(
/* FP:lib.rs-0154 */     declare_combined_early_lint_pass,
/* FP:lib.rs-0155 */     [
/* FP:lib.rs-0156 */         pub BuiltinCombinedPreExpansionLintPass,
/* FP:lib.rs-0157 */         [
/* FP:lib.rs-0158 */             KeywordIdents: KeywordIdents,
/* FP:lib.rs-0159 */         ]
/* FP:lib.rs-0160 */     ]
/* FP:lib.rs-0161 */ );
/* FP:lib.rs-0162 */ 
/* FP:lib.rs-0163 */ early_lint_methods!(
/* FP:lib.rs-0164 */     declare_combined_early_lint_pass,
/* FP:lib.rs-0165 */     [
/* FP:lib.rs-0166 */         pub BuiltinCombinedEarlyLintPass,
/* FP:lib.rs-0167 */         [
/* FP:lib.rs-0168 */             UnusedParens: UnusedParens::default(),
/* FP:lib.rs-0169 */             UnusedBraces: UnusedBraces,
/* FP:lib.rs-0170 */             UnusedImportBraces: UnusedImportBraces,
/* FP:lib.rs-0171 */             UnsafeCode: UnsafeCode,
/* FP:lib.rs-0172 */             SpecialModuleName: SpecialModuleName,
/* FP:lib.rs-0173 */             AnonymousParameters: AnonymousParameters,
/* FP:lib.rs-0174 */             EllipsisInclusiveRangePatterns: EllipsisInclusiveRangePatterns::default(),
/* FP:lib.rs-0175 */             NonCamelCaseTypes: NonCamelCaseTypes,
/* FP:lib.rs-0176 */             WhileTrue: WhileTrue,
/* FP:lib.rs-0177 */             NonAsciiIdents: NonAsciiIdents,
/* FP:lib.rs-0178 */             IncompleteInternalFeatures: IncompleteInternalFeatures,
/* FP:lib.rs-0179 */             RedundantSemicolons: RedundantSemicolons,
/* FP:lib.rs-0180 */             UnusedDocComment: UnusedDocComment,
/* FP:lib.rs-0181 */             Expr2024: Expr2024,
/* FP:lib.rs-0182 */             Precedence: Precedence,
/* FP:lib.rs-0183 */             DoubleNegations: DoubleNegations,
/* FP:lib.rs-0184 */         ]
/* FP:lib.rs-0185 */     ]
/* FP:lib.rs-0186 */ );
/* FP:lib.rs-0187 */ 
/* FP:lib.rs-0188 */ late_lint_methods!(
/* FP:lib.rs-0189 */     declare_combined_late_lint_pass,
/* FP:lib.rs-0190 */     [
/* FP:lib.rs-0191 */         BuiltinCombinedModuleLateLintPass,
/* FP:lib.rs-0192 */         [
/* FP:lib.rs-0193 */             ForLoopsOverFallibles: ForLoopsOverFallibles,
/* FP:lib.rs-0194 */             DefaultCouldBeDerived: DefaultCouldBeDerived::default(),
/* FP:lib.rs-0195 */             DerefIntoDynSupertrait: DerefIntoDynSupertrait,
/* FP:lib.rs-0196 */             DropForgetUseless: DropForgetUseless,
/* FP:lib.rs-0197 */             ImproperCTypesLint: ImproperCTypesLint,
/* FP:lib.rs-0198 */             InvalidFromUtf8: InvalidFromUtf8,
/* FP:lib.rs-0199 */             VariantSizeDifferences: VariantSizeDifferences,
/* FP:lib.rs-0200 */             PathStatements: PathStatements,
/* FP:lib.rs-0201 */             LetUnderscore: LetUnderscore,
/* FP:lib.rs-0202 */             InvalidReferenceCasting: InvalidReferenceCasting,
/* FP:lib.rs-0203 */             ImplicitAutorefs: ImplicitAutorefs,
/* FP:lib.rs-0204 */             // Depends on referenced function signatures in expressions
/* FP:lib.rs-0205 */             UnusedResults: UnusedResults,
/* FP:lib.rs-0206 */             UnitBindings: UnitBindings,
/* FP:lib.rs-0207 */             NonUpperCaseGlobals: NonUpperCaseGlobals,
/* FP:lib.rs-0208 */             NonShorthandFieldPatterns: NonShorthandFieldPatterns,
/* FP:lib.rs-0209 */             UnusedAllocation: UnusedAllocation,
/* FP:lib.rs-0210 */             // Depends on types used in type definitions
/* FP:lib.rs-0211 */             MissingCopyImplementations: MissingCopyImplementations,
/* FP:lib.rs-0212 */             // Depends on referenced function signatures in expressions
/* FP:lib.rs-0213 */             PtrNullChecks: PtrNullChecks,
/* FP:lib.rs-0214 */             MutableTransmutes: MutableTransmutes,
/* FP:lib.rs-0215 */             TypeAliasBounds: TypeAliasBounds,
/* FP:lib.rs-0216 */             TrivialConstraints: TrivialConstraints,
/* FP:lib.rs-0217 */             TypeLimits: TypeLimits::new(),
/* FP:lib.rs-0218 */             NonSnakeCase: NonSnakeCase,
/* FP:lib.rs-0219 */             InvalidNoMangleItems: InvalidNoMangleItems,
/* FP:lib.rs-0220 */             // Depends on effective visibilities
/* FP:lib.rs-0221 */             UnreachablePub: UnreachablePub,
/* FP:lib.rs-0222 */             ExplicitOutlivesRequirements: ExplicitOutlivesRequirements,
/* FP:lib.rs-0223 */             InvalidValue: InvalidValue,
/* FP:lib.rs-0224 */             DerefNullPtr: DerefNullPtr,
/* FP:lib.rs-0225 */             UnstableFeatures: UnstableFeatures,
/* FP:lib.rs-0226 */             UngatedAsyncFnTrackCaller: UngatedAsyncFnTrackCaller,
/* FP:lib.rs-0227 */             ShadowedIntoIter: ShadowedIntoIter,
/* FP:lib.rs-0228 */             DropTraitConstraints: DropTraitConstraints,
/* FP:lib.rs-0229 */             DanglingPointers: DanglingPointers,
/* FP:lib.rs-0230 */             NonPanicFmt: NonPanicFmt,
/* FP:lib.rs-0231 */             NoopMethodCall: NoopMethodCall,
/* FP:lib.rs-0232 */             EnumIntrinsicsNonEnums: EnumIntrinsicsNonEnums,
/* FP:lib.rs-0233 */             InvalidAtomicOrdering: InvalidAtomicOrdering,
/* FP:lib.rs-0234 */             AsmLabels: AsmLabels,
/* FP:lib.rs-0235 */             OpaqueHiddenInferredBound: OpaqueHiddenInferredBound,
/* FP:lib.rs-0236 */             MultipleSupertraitUpcastable: MultipleSupertraitUpcastable,
/* FP:lib.rs-0237 */             MapUnitFn: MapUnitFn,
/* FP:lib.rs-0238 */             MissingDebugImplementations: MissingDebugImplementations,
/* FP:lib.rs-0239 */             MissingDoc: MissingDoc,
/* FP:lib.rs-0240 */             AsyncClosureUsage: AsyncClosureUsage,
/* FP:lib.rs-0241 */             AsyncFnInTrait: AsyncFnInTrait,
/* FP:lib.rs-0242 */             NonLocalDefinitions: NonLocalDefinitions::default(),
/* FP:lib.rs-0243 */             ImplTraitOvercaptures: ImplTraitOvercaptures,
/* FP:lib.rs-0244 */             IfLetRescope: IfLetRescope::default(),
/* FP:lib.rs-0245 */             StaticMutRefs: StaticMutRefs,
/* FP:lib.rs-0246 */             UnqualifiedLocalImports: UnqualifiedLocalImports,
/* FP:lib.rs-0247 */             CheckTransmutes: CheckTransmutes,
/* FP:lib.rs-0248 */             LifetimeSyntax: LifetimeSyntax,
/* FP:lib.rs-0249 */         ]
/* FP:lib.rs-0250 */     ]
/* FP:lib.rs-0251 */ );
/* FP:lib.rs-0252 */ 
/* FP:lib.rs-0253 */ pub fn new_lint_store(internal_lints: bool) -> LintStore {
/* FP:lib.rs-0254 */     let mut lint_store = LintStore::new();
/* FP:lib.rs-0255 */ 
/* FP:lib.rs-0256 */     register_builtins(&mut lint_store);
/* FP:lib.rs-0257 */     if internal_lints {
/* FP:lib.rs-0258 */         register_internals(&mut lint_store);
/* FP:lib.rs-0259 */     }
/* FP:lib.rs-0260 */ 
/* FP:lib.rs-0261 */     lint_store
/* FP:lib.rs-0262 */ }
/* FP:lib.rs-0263 */ 
/* FP:lib.rs-0264 */ /// Tell the `LintStore` about all the built-in lints (the ones
/* FP:lib.rs-0265 */ /// defined in this crate and the ones defined in
/* FP:lib.rs-0266 */ /// `crate::rustc_session::lint::builtin`).
/* FP:lib.rs-0267 */ fn register_builtins(store: &mut LintStore) {
/* FP:lib.rs-0268 */     macro_rules! add_lint_group {
/* FP:lib.rs-0269 */         ($name:expr, $($lint:ident),*) => (
/* FP:lib.rs-0270 */             store.register_group(false, $name, None, vec![$(LintId::of($lint)),*]);
/* FP:lib.rs-0271 */         )
/* FP:lib.rs-0272 */     }
/* FP:lib.rs-0273 */ 
/* FP:lib.rs-0274 */     store.register_lints(&BuiltinCombinedPreExpansionLintPass::get_lints());
/* FP:lib.rs-0275 */     store.register_lints(&BuiltinCombinedEarlyLintPass::get_lints());
/* FP:lib.rs-0276 */     store.register_lints(&BuiltinCombinedModuleLateLintPass::get_lints());
/* FP:lib.rs-0277 */     store.register_lints(&foreign_modules::get_lints());
/* FP:lib.rs-0278 */     store.register_lints(&HardwiredLints::lint_vec());
/* FP:lib.rs-0279 */ 
/* FP:lib.rs-0280 */     add_lint_group!(
/* FP:lib.rs-0281 */         "nonstandard_style",
/* FP:lib.rs-0282 */         NON_CAMEL_CASE_TYPES,
/* FP:lib.rs-0283 */         NON_SNAKE_CASE,
/* FP:lib.rs-0284 */         NON_UPPER_CASE_GLOBALS
/* FP:lib.rs-0285 */     );
/* FP:lib.rs-0286 */ 
/* FP:lib.rs-0287 */     add_lint_group!(
/* FP:lib.rs-0288 */         "unused",
/* FP:lib.rs-0289 */         UNUSED_IMPORTS,
/* FP:lib.rs-0290 */         UNUSED_VARIABLES,
/* FP:lib.rs-0291 */         UNUSED_ASSIGNMENTS,
/* FP:lib.rs-0292 */         DEAD_CODE,
/* FP:lib.rs-0293 */         UNUSED_MUT,
/* FP:lib.rs-0294 */         UNREACHABLE_CODE,
/* FP:lib.rs-0295 */         UNREACHABLE_PATTERNS,
/* FP:lib.rs-0296 */         UNUSED_MUST_USE,
/* FP:lib.rs-0297 */         UNUSED_UNSAFE,
/* FP:lib.rs-0298 */         PATH_STATEMENTS,
/* FP:lib.rs-0299 */         UNUSED_ATTRIBUTES,
/* FP:lib.rs-0300 */         UNUSED_MACROS,
/* FP:lib.rs-0301 */         UNUSED_MACRO_RULES,
/* FP:lib.rs-0302 */         UNUSED_ALLOCATION,
/* FP:lib.rs-0303 */         UNUSED_DOC_COMMENTS,
/* FP:lib.rs-0304 */         UNUSED_EXTERN_CRATES,
/* FP:lib.rs-0305 */         UNUSED_FEATURES,
/* FP:lib.rs-0306 */         UNUSED_LABELS,
/* FP:lib.rs-0307 */         UNUSED_PARENS,
/* FP:lib.rs-0308 */         UNUSED_BRACES,
/* FP:lib.rs-0309 */         REDUNDANT_SEMICOLONS,
/* FP:lib.rs-0310 */         MAP_UNIT_FN
/* FP:lib.rs-0311 */     );
/* FP:lib.rs-0312 */ 
/* FP:lib.rs-0313 */     add_lint_group!("let_underscore", LET_UNDERSCORE_DROP, LET_UNDERSCORE_LOCK);
/* FP:lib.rs-0314 */ 
/* FP:lib.rs-0315 */     add_lint_group!(
/* FP:lib.rs-0316 */         "rust_2018_idioms",
/* FP:lib.rs-0317 */         BARE_TRAIT_OBJECTS,
/* FP:lib.rs-0318 */         UNUSED_EXTERN_CRATES,
/* FP:lib.rs-0319 */         ELLIPSIS_INCLUSIVE_RANGE_PATTERNS,
/* FP:lib.rs-0320 */         ELIDED_LIFETIMES_IN_PATHS,
/* FP:lib.rs-0321 */         EXPLICIT_OUTLIVES_REQUIREMENTS // FIXME(#52665, #47816) not always applicable and not all
/* FP:lib.rs-0322 */                                        // macros are ready for this yet.
/* FP:lib.rs-0323 */                                        // UNREACHABLE_PUB,
/* FP:lib.rs-0324 */ 
/* FP:lib.rs-0325 */                                        // FIXME macro crates are not up for this yet, too much
/* FP:lib.rs-0326 */                                        // breakage is seen if we try to encourage this lint.
/* FP:lib.rs-0327 */                                        // MACRO_USE_EXTERN_CRATE
/* FP:lib.rs-0328 */     );
/* FP:lib.rs-0329 */ 
/* FP:lib.rs-0330 */     add_lint_group!("keyword_idents", KEYWORD_IDENTS_2018, KEYWORD_IDENTS_2024);
/* FP:lib.rs-0331 */ 
/* FP:lib.rs-0332 */     add_lint_group!(
/* FP:lib.rs-0333 */         "refining_impl_trait",
/* FP:lib.rs-0334 */         REFINING_IMPL_TRAIT_REACHABLE,
/* FP:lib.rs-0335 */         REFINING_IMPL_TRAIT_INTERNAL
/* FP:lib.rs-0336 */     );
/* FP:lib.rs-0337 */ 
/* FP:lib.rs-0338 */     add_lint_group!("deprecated_safe", DEPRECATED_SAFE_2024);
/* FP:lib.rs-0339 */ 
/* FP:lib.rs-0340 */     add_lint_group!(
/* FP:lib.rs-0341 */         "unknown_or_malformed_diagnostic_attributes",
/* FP:lib.rs-0342 */         MALFORMED_DIAGNOSTIC_ATTRIBUTES,
/* FP:lib.rs-0343 */         MALFORMED_DIAGNOSTIC_FORMAT_LITERALS,
/* FP:lib.rs-0344 */         MISPLACED_DIAGNOSTIC_ATTRIBUTES,
/* FP:lib.rs-0345 */         UNKNOWN_DIAGNOSTIC_ATTRIBUTES
/* FP:lib.rs-0346 */     );
/* FP:lib.rs-0347 */ 
/* FP:lib.rs-0348 */     // Register renamed and removed lints.
/* FP:lib.rs-0349 */     store.register_renamed("single_use_lifetime", "single_use_lifetimes");
/* FP:lib.rs-0350 */     store.register_renamed("elided_lifetime_in_path", "elided_lifetimes_in_paths");
/* FP:lib.rs-0351 */     store.register_renamed("bare_trait_object", "bare_trait_objects");
/* FP:lib.rs-0352 */     store.register_renamed("unstable_name_collision", "unstable_name_collisions");
/* FP:lib.rs-0353 */     store.register_renamed("unused_doc_comment", "unused_doc_comments");
/* FP:lib.rs-0354 */     store.register_renamed("async_idents", "keyword_idents_2018");
/* FP:lib.rs-0355 */     store.register_renamed("exceeding_bitshifts", "arithmetic_overflow");
/* FP:lib.rs-0356 */     store.register_renamed("redundant_semicolon", "redundant_semicolons");
/* FP:lib.rs-0357 */     store.register_renamed("overlapping_patterns", "overlapping_range_endpoints");
/* FP:lib.rs-0358 */     store.register_renamed("disjoint_capture_migration", "rust_2021_incompatible_closure_captures");
/* FP:lib.rs-0359 */     store.register_renamed("or_patterns_back_compat", "rust_2021_incompatible_or_patterns");
/* FP:lib.rs-0360 */     store.register_renamed("non_fmt_panic", "non_fmt_panics");
/* FP:lib.rs-0361 */     store.register_renamed("unused_tuple_struct_fields", "dead_code");
/* FP:lib.rs-0362 */     store.register_renamed("static_mut_ref", "static_mut_refs");
/* FP:lib.rs-0363 */     store.register_renamed("temporary_cstring_as_ptr", "dangling_pointers_from_temporaries");
/* FP:lib.rs-0364 */     store.register_renamed("elided_named_lifetimes", "mismatched_lifetime_syntaxes");
/* FP:lib.rs-0365 */ 
/* FP:lib.rs-0366 */     // These were moved to tool lints, but rustc still sees them when compiling normally, before
/* FP:lib.rs-0367 */     // tool lints are registered, so `check_tool_name_for_backwards_compat` doesn't work. Use
/* FP:lib.rs-0368 */     // `register_removed` explicitly.
/* FP:lib.rs-0369 */     const RUSTDOC_LINTS: &[&str] = &[
/* FP:lib.rs-0370 */         "broken_intra_doc_links",
/* FP:lib.rs-0371 */         "private_intra_doc_links",
/* FP:lib.rs-0372 */         "missing_crate_level_docs",
/* FP:lib.rs-0373 */         "missing_doc_code_examples",
/* FP:lib.rs-0374 */         "private_doc_tests",
/* FP:lib.rs-0375 */         "invalid_codeblock_attributes",
/* FP:lib.rs-0376 */         "invalid_html_tags",
/* FP:lib.rs-0377 */         "non_autolinks",
/* FP:lib.rs-0378 */     ];
/* FP:lib.rs-0379 */     for rustdoc_lint in RUSTDOC_LINTS {
/* FP:lib.rs-0380 */         store.register_ignored(rustdoc_lint);
/* FP:lib.rs-0381 */     }
/* FP:lib.rs-0382 */     store.register_removed(
/* FP:lib.rs-0383 */         "intra_doc_link_resolution_failure",
/* FP:lib.rs-0384 */         "use `rustdoc::broken_intra_doc_links` instead",
/* FP:lib.rs-0385 */     );
/* FP:lib.rs-0386 */     store.register_removed("rustdoc", "use `rustdoc::all` instead");
/* FP:lib.rs-0387 */ 
/* FP:lib.rs-0388 */     store.register_removed("unknown_features", "replaced by an error");
/* FP:lib.rs-0389 */     store.register_removed("unsigned_negation", "replaced by negate_unsigned feature gate");
/* FP:lib.rs-0390 */     store.register_removed("negate_unsigned", "cast a signed value instead");
/* FP:lib.rs-0391 */     store.register_removed("raw_pointer_derive", "using derive with raw pointers is ok");
/* FP:lib.rs-0392 */     // Register lint group aliases.
/* FP:lib.rs-0393 */     store.register_group_alias("nonstandard_style", "bad_style");
/* FP:lib.rs-0394 */     // This was renamed to `raw_pointer_derive`, which was then removed,
/* FP:lib.rs-0395 */     // so it is also considered removed.
/* FP:lib.rs-0396 */     store.register_removed("raw_pointer_deriving", "using derive with raw pointers is ok");
/* FP:lib.rs-0397 */     store.register_removed("drop_with_repr_extern", "drop flags have been removed");
/* FP:lib.rs-0398 */     store.register_removed("fat_ptr_transmutes", "was accidentally removed back in 2014");
/* FP:lib.rs-0399 */     store.register_removed("deprecated_attr", "use `deprecated` instead");
/* FP:lib.rs-0400 */     store.register_removed(
/* FP:lib.rs-0401 */         "transmute_from_fn_item_types",
/* FP:lib.rs-0402 */         "always cast functions before transmuting them",
/* FP:lib.rs-0403 */     );
/* FP:lib.rs-0404 */     store.register_removed(
/* FP:lib.rs-0405 */         "hr_lifetime_in_assoc_type",
/* FP:lib.rs-0406 */         "converted into hard error, see issue #33685 \
/* FP:lib.rs-0407 */          <https://github.com/rust-lang/rust/issues/33685> for more information",
/* FP:lib.rs-0408 */     );
/* FP:lib.rs-0409 */     store.register_removed(
/* FP:lib.rs-0410 */         "inaccessible_extern_crate",
/* FP:lib.rs-0411 */         "converted into hard error, see issue #36886 \
/* FP:lib.rs-0412 */          <https://github.com/rust-lang/rust/issues/36886> for more information",
/* FP:lib.rs-0413 */     );
/* FP:lib.rs-0414 */     store.register_removed(
/* FP:lib.rs-0415 */         "super_or_self_in_global_path",
/* FP:lib.rs-0416 */         "converted into hard error, see issue #36888 \
/* FP:lib.rs-0417 */          <https://github.com/rust-lang/rust/issues/36888> for more information",
/* FP:lib.rs-0418 */     );
/* FP:lib.rs-0419 */     store.register_removed(
/* FP:lib.rs-0420 */         "overlapping_inherent_impls",
/* FP:lib.rs-0421 */         "converted into hard error, see issue #36889 \
/* FP:lib.rs-0422 */          <https://github.com/rust-lang/rust/issues/36889> for more information",
/* FP:lib.rs-0423 */     );
/* FP:lib.rs-0424 */     store.register_removed(
/* FP:lib.rs-0425 */         "illegal_floating_point_constant_pattern",
/* FP:lib.rs-0426 */         "converted into hard error, see issue #36890 \
/* FP:lib.rs-0427 */          <https://github.com/rust-lang/rust/issues/36890> for more information",
/* FP:lib.rs-0428 */     );
/* FP:lib.rs-0429 */     store.register_removed(
/* FP:lib.rs-0430 */         "illegal_struct_or_enum_constant_pattern",
/* FP:lib.rs-0431 */         "converted into hard error, see issue #36891 \
/* FP:lib.rs-0432 */          <https://github.com/rust-lang/rust/issues/36891> for more information",
/* FP:lib.rs-0433 */     );
/* FP:lib.rs-0434 */     store.register_removed(
/* FP:lib.rs-0435 */         "lifetime_underscore",
/* FP:lib.rs-0436 */         "converted into hard error, see issue #36892 \
/* FP:lib.rs-0437 */          <https://github.com/rust-lang/rust/issues/36892> for more information",
/* FP:lib.rs-0438 */     );
/* FP:lib.rs-0439 */     store.register_removed(
/* FP:lib.rs-0440 */         "extra_requirement_in_impl",
/* FP:lib.rs-0441 */         "converted into hard error, see issue #37166 \
/* FP:lib.rs-0442 */          <https://github.com/rust-lang/rust/issues/37166> for more information",
/* FP:lib.rs-0443 */     );
/* FP:lib.rs-0444 */     store.register_removed(
/* FP:lib.rs-0445 */         "legacy_imports",
/* FP:lib.rs-0446 */         "converted into hard error, see issue #38260 \
/* FP:lib.rs-0447 */          <https://github.com/rust-lang/rust/issues/38260> for more information",
/* FP:lib.rs-0448 */     );
/* FP:lib.rs-0449 */     store.register_removed(
/* FP:lib.rs-0450 */         "coerce_never",
/* FP:lib.rs-0451 */         "converted into hard error, see issue #48950 \
/* FP:lib.rs-0452 */          <https://github.com/rust-lang/rust/issues/48950> for more information",
/* FP:lib.rs-0453 */     );
/* FP:lib.rs-0454 */     store.register_removed(
/* FP:lib.rs-0455 */         "resolve_trait_on_defaulted_unit",
/* FP:lib.rs-0456 */         "converted into hard error, see issue #48950 \
/* FP:lib.rs-0457 */          <https://github.com/rust-lang/rust/issues/48950> for more information",
/* FP:lib.rs-0458 */     );
/* FP:lib.rs-0459 */     store.register_removed(
/* FP:lib.rs-0460 */         "private_no_mangle_fns",
/* FP:lib.rs-0461 */         "no longer a warning, `#[unsafe(no_mangle)]` functions always exported",
/* FP:lib.rs-0462 */     );
/* FP:lib.rs-0463 */     store.register_removed(
/* FP:lib.rs-0464 */         "private_no_mangle_statics",
/* FP:lib.rs-0465 */         "no longer a warning, `#[unsafe(no_mangle)]` statics always exported",
/* FP:lib.rs-0466 */     );
/* FP:lib.rs-0467 */     store.register_removed("bad_repr", "replaced with a generic attribute input check");
/* FP:lib.rs-0468 */     store.register_removed(
/* FP:lib.rs-0469 */         "duplicate_matcher_binding_name",
/* FP:lib.rs-0470 */         "converted into hard error, see issue #57742 \
/* FP:lib.rs-0471 */          <https://github.com/rust-lang/rust/issues/57742> for more information",
/* FP:lib.rs-0472 */     );
/* FP:lib.rs-0473 */     store.register_removed(
/* FP:lib.rs-0474 */         "incoherent_fundamental_impls",
/* FP:lib.rs-0475 */         "converted into hard error, see issue #46205 \
/* FP:lib.rs-0476 */          <https://github.com/rust-lang/rust/issues/46205> for more information",
/* FP:lib.rs-0477 */     );
/* FP:lib.rs-0478 */     store.register_removed(
/* FP:lib.rs-0479 */         "legacy_constructor_visibility",
/* FP:lib.rs-0480 */         "converted into hard error, see issue #39207 \
/* FP:lib.rs-0481 */          <https://github.com/rust-lang/rust/issues/39207> for more information",
/* FP:lib.rs-0482 */     );
/* FP:lib.rs-0483 */     store.register_removed(
/* FP:lib.rs-0484 */         "legacy_directory_ownership",
/* FP:lib.rs-0485 */         "converted into hard error, see issue #37872 \
/* FP:lib.rs-0486 */          <https://github.com/rust-lang/rust/issues/37872> for more information",
/* FP:lib.rs-0487 */     );
/* FP:lib.rs-0488 */     store.register_removed(
/* FP:lib.rs-0489 */         "safe_extern_statics",
/* FP:lib.rs-0490 */         "converted into hard error, see issue #36247 \
/* FP:lib.rs-0491 */          <https://github.com/rust-lang/rust/issues/36247> for more information",
/* FP:lib.rs-0492 */     );
/* FP:lib.rs-0493 */     store.register_removed(
/* FP:lib.rs-0494 */         "parenthesized_params_in_types_and_modules",
/* FP:lib.rs-0495 */         "converted into hard error, see issue #42238 \
/* FP:lib.rs-0496 */          <https://github.com/rust-lang/rust/issues/42238> for more information",
/* FP:lib.rs-0497 */     );
/* FP:lib.rs-0498 */     store.register_removed(
/* FP:lib.rs-0499 */         "duplicate_macro_exports",
/* FP:lib.rs-0500 */         "converted into hard error, see issue #35896 \
/* FP:lib.rs-0501 */          <https://github.com/rust-lang/rust/issues/35896> for more information",
/* FP:lib.rs-0502 */     );
/* FP:lib.rs-0503 */     store.register_removed(
/* FP:lib.rs-0504 */         "nested_impl_trait",
/* FP:lib.rs-0505 */         "converted into hard error, see issue #59014 \
/* FP:lib.rs-0506 */          <https://github.com/rust-lang/rust/issues/59014> for more information",
/* FP:lib.rs-0507 */     );
/* FP:lib.rs-0508 */     store.register_removed("plugin_as_library", "plugins have been deprecated and retired");
/* FP:lib.rs-0509 */     store.register_removed(
/* FP:lib.rs-0510 */         "unsupported_naked_functions",
/* FP:lib.rs-0511 */         "converted into hard error, see RFC 2972 \
/* FP:lib.rs-0512 */          <https://github.com/rust-lang/rfcs/blob/master/text/2972-constrained-naked.md> for more information",
/* FP:lib.rs-0513 */     );
/* FP:lib.rs-0514 */     store.register_removed(
/* FP:lib.rs-0515 */         "mutable_borrow_reservation_conflict",
/* FP:lib.rs-0516 */         "now allowed, see issue #59159 \
/* FP:lib.rs-0517 */          <https://github.com/rust-lang/rust/issues/59159> for more information",
/* FP:lib.rs-0518 */     );
/* FP:lib.rs-0519 */     store.register_removed(
/* FP:lib.rs-0520 */         "const_err",
/* FP:lib.rs-0521 */         "converted into hard error, see issue #71800 \
/* FP:lib.rs-0522 */          <https://github.com/rust-lang/rust/issues/71800> for more information",
/* FP:lib.rs-0523 */     );
/* FP:lib.rs-0524 */     store.register_removed(
/* FP:lib.rs-0525 */         "safe_packed_borrows",
/* FP:lib.rs-0526 */         "converted into hard error, see issue #82523 \
/* FP:lib.rs-0527 */          <https://github.com/rust-lang/rust/issues/82523> for more information",
/* FP:lib.rs-0528 */     );
/* FP:lib.rs-0529 */     store.register_removed(
/* FP:lib.rs-0530 */         "unaligned_references",
/* FP:lib.rs-0531 */         "converted into hard error, see issue #82523 \
/* FP:lib.rs-0532 */          <https://github.com/rust-lang/rust/issues/82523> for more information",
/* FP:lib.rs-0533 */     );
/* FP:lib.rs-0534 */     store.register_removed(
/* FP:lib.rs-0535 */         "private_in_public",
/* FP:lib.rs-0536 */         "replaced with another group of lints, see RFC \
/* FP:lib.rs-0537 */          <https://rust-lang.github.io/rfcs/2145-type-privacy.html> for more information",
/* FP:lib.rs-0538 */     );
/* FP:lib.rs-0539 */     store.register_removed(
/* FP:lib.rs-0540 */         "invalid_alignment",
/* FP:lib.rs-0541 */         "converted into hard error, see PR #104616 \
/* FP:lib.rs-0542 */          <https://github.com/rust-lang/rust/pull/104616> for more information",
/* FP:lib.rs-0543 */     );
/* FP:lib.rs-0544 */     store.register_removed(
/* FP:lib.rs-0545 */         "implied_bounds_entailment",
/* FP:lib.rs-0546 */         "converted into hard error, see PR #117984 \
/* FP:lib.rs-0547 */         <https://github.com/rust-lang/rust/pull/117984> for more information",
/* FP:lib.rs-0548 */     );
/* FP:lib.rs-0549 */     store.register_removed(
/* FP:lib.rs-0550 */         "coinductive_overlap_in_coherence",
/* FP:lib.rs-0551 */         "converted into hard error, see PR #118649 \
/* FP:lib.rs-0552 */          <https://github.com/rust-lang/rust/pull/118649> for more information",
/* FP:lib.rs-0553 */     );
/* FP:lib.rs-0554 */     store.register_removed(
/* FP:lib.rs-0555 */         "illegal_floating_point_literal_pattern",
/* FP:lib.rs-0556 */         "no longer a warning, float patterns behave the same as `==`",
/* FP:lib.rs-0557 */     );
/* FP:lib.rs-0558 */     store.register_removed(
/* FP:lib.rs-0559 */         "nontrivial_structural_match",
/* FP:lib.rs-0560 */         "no longer needed, see RFC #3535 \
/* FP:lib.rs-0561 */          <https://rust-lang.github.io/rfcs/3535-constants-in-patterns.html> for more information",
/* FP:lib.rs-0562 */     );
/* FP:lib.rs-0563 */     store.register_removed(
/* FP:lib.rs-0564 */         "suspicious_auto_trait_impls",
/* FP:lib.rs-0565 */         "no longer needed, see issue #93367 \
/* FP:lib.rs-0566 */          <https://github.com/rust-lang/rust/issues/93367> for more information",
/* FP:lib.rs-0567 */     );
/* FP:lib.rs-0568 */     store.register_removed(
/* FP:lib.rs-0569 */         "const_patterns_without_partial_eq",
/* FP:lib.rs-0570 */         "converted into hard error, see RFC #3535 \
/* FP:lib.rs-0571 */          <https://rust-lang.github.io/rfcs/3535-constants-in-patterns.html> for more information",
/* FP:lib.rs-0572 */     );
/* FP:lib.rs-0573 */     store.register_removed(
/* FP:lib.rs-0574 */         "indirect_structural_match",
/* FP:lib.rs-0575 */         "converted into hard error, see RFC #3535 \
/* FP:lib.rs-0576 */          <https://rust-lang.github.io/rfcs/3535-constants-in-patterns.html> for more information",
/* FP:lib.rs-0577 */     );
/* FP:lib.rs-0578 */     store.register_removed(
/* FP:lib.rs-0579 */         "deprecated_cfg_attr_crate_type_name",
/* FP:lib.rs-0580 */         "converted into hard error, see issue #91632 \
/* FP:lib.rs-0581 */          <https://github.com/rust-lang/rust/issues/91632> for more information",
/* FP:lib.rs-0582 */     );
/* FP:lib.rs-0583 */     store.register_removed(
/* FP:lib.rs-0584 */         "pointer_structural_match",
/* FP:lib.rs-0585 */         "converted into hard error, see RFC #3535 \
/* FP:lib.rs-0586 */          <https://rust-lang.github.io/rfcs/3535-constants-in-patterns.html> for more information",
/* FP:lib.rs-0587 */     );
/* FP:lib.rs-0588 */     store.register_removed(
/* FP:lib.rs-0589 */         "box_pointers",
/* FP:lib.rs-0590 */         "it does not detect other kinds of allocations, and existed only for historical reasons",
/* FP:lib.rs-0591 */     );
/* FP:lib.rs-0592 */     store.register_removed(
/* FP:lib.rs-0593 */         "byte_slice_in_packed_struct_with_derive",
/* FP:lib.rs-0594 */         "converted into hard error, see issue #107457 \
/* FP:lib.rs-0595 */          <https://github.com/rust-lang/rust/issues/107457> for more information",
/* FP:lib.rs-0596 */     );
/* FP:lib.rs-0597 */     store.register_removed("writes_through_immutable_pointer", "converted into hard error");
/* FP:lib.rs-0598 */     store.register_removed(
/* FP:lib.rs-0599 */         "const_eval_mutable_ptr_in_final_value",
/* FP:lib.rs-0600 */         "partially allowed now, otherwise turned into a hard error",
/* FP:lib.rs-0601 */     );
/* FP:lib.rs-0602 */     store.register_removed(
/* FP:lib.rs-0603 */         "where_clauses_object_safety",
/* FP:lib.rs-0604 */         "converted into hard error, see PR #125380 \
/* FP:lib.rs-0605 */          <https://github.com/rust-lang/rust/pull/125380> for more information",
/* FP:lib.rs-0606 */     );
/* FP:lib.rs-0607 */     store.register_removed(
/* FP:lib.rs-0608 */         "cenum_impl_drop_cast",
/* FP:lib.rs-0609 */         "converted into hard error, \
/* FP:lib.rs-0610 */          see <https://github.com/rust-lang/rust/issues/73333> for more information",
/* FP:lib.rs-0611 */     );
/* FP:lib.rs-0612 */     store.register_removed(
/* FP:lib.rs-0613 */         "ptr_cast_add_auto_to_object",
/* FP:lib.rs-0614 */         "converted into hard error, see issue #127323 \
/* FP:lib.rs-0615 */          <https://github.com/rust-lang/rust/issues/127323> for more information",
/* FP:lib.rs-0616 */     );
/* FP:lib.rs-0617 */     store.register_removed("unsupported_fn_ptr_calling_conventions", "converted into hard error");
/* FP:lib.rs-0618 */     store.register_removed(
/* FP:lib.rs-0619 */         "undefined_naked_function_abi",
/* FP:lib.rs-0620 */         "converted into hard error, see PR #139001 \
/* FP:lib.rs-0621 */          <https://github.com/rust-lang/rust/issues/139001> for more information",
/* FP:lib.rs-0622 */     );
/* FP:lib.rs-0623 */     store.register_removed(
/* FP:lib.rs-0624 */         "abi_unsupported_vector_types",
/* FP:lib.rs-0625 */         "converted into hard error, \
/* FP:lib.rs-0626 */          see <https://github.com/rust-lang/rust/issues/116558> for more information",
/* FP:lib.rs-0627 */     );
/* FP:lib.rs-0628 */     store.register_removed(
/* FP:lib.rs-0629 */         "missing_fragment_specifier",
/* FP:lib.rs-0630 */         "converted into hard error, \
/* FP:lib.rs-0631 */          see <https://github.com/rust-lang/rust/issues/40107> for more information",
/* FP:lib.rs-0632 */     );
/* FP:lib.rs-0633 */     store.register_removed("wasm_c_abi", "the wasm C ABI has been fixed");
/* FP:lib.rs-0634 */ }
/* FP:lib.rs-0635 */ 
/* FP:lib.rs-0636 */ fn register_internals(store: &mut LintStore) {
/* FP:lib.rs-0637 */     store.register_lints(&LintPassImpl::lint_vec());
/* FP:lib.rs-0638 */     store.register_early_pass(|| Box::new(LintPassImpl));
/* FP:lib.rs-0639 */     store.register_lints(&DefaultHashTypes::lint_vec());
/* FP:lib.rs-0640 */     store.register_late_mod_pass(|_| Box::new(DefaultHashTypes));
/* FP:lib.rs-0641 */     store.register_lints(&QueryStability::lint_vec());
/* FP:lib.rs-0642 */     store.register_late_mod_pass(|_| Box::new(QueryStability));
/* FP:lib.rs-0643 */     store.register_lints(&TyTyKind::lint_vec());
/* FP:lib.rs-0644 */     store.register_late_mod_pass(|_| Box::new(TyTyKind));
/* FP:lib.rs-0645 */     store.register_lints(&TypeIr::lint_vec());
/* FP:lib.rs-0646 */     store.register_late_mod_pass(|_| Box::new(TypeIr));
/* FP:lib.rs-0647 */     store.register_lints(&Diagnostics::lint_vec());
/* FP:lib.rs-0648 */     store.register_late_mod_pass(|_| Box::new(Diagnostics));
/* FP:lib.rs-0649 */     store.register_lints(&BadOptAccess::lint_vec());
/* FP:lib.rs-0650 */     store.register_late_mod_pass(|_| Box::new(BadOptAccess));
/* FP:lib.rs-0651 */     store.register_lints(&PassByValue::lint_vec());
/* FP:lib.rs-0652 */     store.register_late_mod_pass(|_| Box::new(PassByValue));
/* FP:lib.rs-0653 */     store.register_lints(&SpanUseEqCtxt::lint_vec());
/* FP:lib.rs-0654 */     store.register_late_mod_pass(|_| Box::new(SpanUseEqCtxt));
/* FP:lib.rs-0655 */     store.register_lints(&SymbolInternStringLiteral::lint_vec());
/* FP:lib.rs-0656 */     store.register_late_mod_pass(|_| Box::new(SymbolInternStringLiteral));
/* FP:lib.rs-0657 */     // FIXME(davidtwco): deliberately do not include `UNTRANSLATABLE_DIAGNOSTIC` and
/* FP:lib.rs-0658 */     // `DIAGNOSTIC_OUTSIDE_OF_IMPL` here because `-Wrustc::internal` is provided to every crate and
/* FP:lib.rs-0659 */     // these lints will trigger all of the time - change this once migration to diagnostic structs
/* FP:lib.rs-0660 */     // and translation is completed
/* FP:lib.rs-0661 */     store.register_group(
/* FP:lib.rs-0662 */         false,
/* FP:lib.rs-0663 */         "rustc::internal",
/* FP:lib.rs-0664 */         None,
/* FP:lib.rs-0665 */         vec![
/* FP:lib.rs-0666 */             LintId::of(DEFAULT_HASH_TYPES),
/* FP:lib.rs-0667 */             LintId::of(POTENTIAL_QUERY_INSTABILITY),
/* FP:lib.rs-0668 */             LintId::of(UNTRACKED_QUERY_INFORMATION),
/* FP:lib.rs-0669 */             LintId::of(USAGE_OF_TY_TYKIND),
/* FP:lib.rs-0670 */             LintId::of(PASS_BY_VALUE),
/* FP:lib.rs-0671 */             LintId::of(LINT_PASS_IMPL_WITHOUT_MACRO),
/* FP:lib.rs-0672 */             LintId::of(USAGE_OF_QUALIFIED_TY),
/* FP:lib.rs-0673 */             LintId::of(NON_GLOB_IMPORT_OF_TYPE_IR_INHERENT),
/* FP:lib.rs-0674 */             LintId::of(USAGE_OF_TYPE_IR_INHERENT),
/* FP:lib.rs-0675 */             LintId::of(USAGE_OF_TYPE_IR_TRAITS),
/* FP:lib.rs-0676 */             LintId::of(BAD_OPT_ACCESS),
/* FP:lib.rs-0677 */             LintId::of(SPAN_USE_EQ_CTXT),
/* FP:lib.rs-0678 */             LintId::of(DIRECT_USE_OF_RUSTC_TYPE_IR),
/* FP:lib.rs-0679 */         ],
/* FP:lib.rs-0680 */     );
/* FP:lib.rs-0681 */ }
/* FP:lib.rs-0682 */ 
/* FP:lib.rs-0683 */ #[cfg(test)]