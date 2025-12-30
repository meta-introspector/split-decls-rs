// Generated macro for pair_unit_variant (function)
macro_rules! Depcrate_derivepair_unit_variant {
() => {
// Module: crate::derive
// Provides: {"pair_unit_variant"}
// Dependencies: {}
# [doc = " Deal with a unit variant."] fn pair_unit_variant (ctx : Ctx , attrs : & ParsedAttributes , v_path : Path ,) -> StratPair { error :: if_present_on_unit_variant (ctx , attrs) ; pair_unit_self (& v_path) }
};
}
