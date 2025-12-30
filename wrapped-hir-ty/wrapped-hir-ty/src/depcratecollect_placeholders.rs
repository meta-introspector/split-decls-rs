// Generated macro for collect_placeholders (function)
macro_rules! Depcratecollect_placeholders {
() => {
// Module: crate
// Provides: {"collect_placeholders"}
// Dependencies: {}
# [doc = " Returns unique placeholders for types and consts contained in `value`."] pub fn collect_placeholders < T > (value : & T , db : & dyn HirDatabase) -> Vec < TypeOrConstParamId > where T : ? Sized + TypeVisitable < Interner > , { let mut collector = PlaceholderCollector { db , placeholders : FxHashSet :: default () } ; _ = value . visit_with (& mut collector , DebruijnIndex :: INNERMOST) ; collector . placeholders . into_iter () . collect () }
};
}
