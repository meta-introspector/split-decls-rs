// Generated macro for LazyInfo (struct)
macro_rules! Depcrate_non_std_lazy_staticsLazyInfo {
() => {
// Module: crate::non_std_lazy_statics
// Provides: {"LazyInfo"}
// Dependencies: {}
struct LazyInfo { # [doc = " Span of the [`hir::Ty`] without including args."] # [doc = " i.e.:"] # [doc = " ```ignore"] # [doc = " static FOO: Lazy<String> = Lazy::new(...);"] # [doc = " //          ^^^^"] # [doc = " ```"] ty_span_no_args : Span , # [doc = " Item on which the lint must be generated."] item_hir_id : HirId , # [doc = " `Span` and `DefId` of calls on `Lazy` type."] # [doc = " i.e.:"] # [doc = " ```ignore"] # [doc = " static FOO: Lazy<String> = Lazy::new(...);"] # [doc = " //                         ^^^^^^^^^"] # [doc = " ```"] calls_span_and_id : FxIndexMap < Span , DefId > , }
};
}
