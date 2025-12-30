// Generated macro for ensure_union_has_strategies (function)
macro_rules! Depcrate_deriveensure_union_has_strategies {
() => {
// Module: crate::derive
// Provides: {"ensure_union_has_strategies"}
// Dependencies: {}
# [doc = " Ensure that there's at least one generatable variant for a union."] fn ensure_union_has_strategies < C > (ctx : Ctx , strats : & StratAcc < C >) { if strats . is_empty () { error :: uninhabited_enum_because_of_skipped_variants (ctx) ; } }
};
}
