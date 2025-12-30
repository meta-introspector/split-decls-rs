// Generated macro for if_anything_specified (function)
macro_rules! Depcrate_errorif_anything_specified {
() => {
// Module: crate::error
// Provides: {"if_anything_specified"}
// Dependencies: {}
# [doc = " Ensures that no attributes were specified on `item`."] pub fn if_anything_specified (ctx : Ctx , attrs : & ParsedAttributes , item : & str) { if_enum_attrs_present (ctx , attrs , item) ; if_strategy_present (ctx , attrs , item) ; if_specified_params (ctx , attrs , item) ; if_specified_filter (ctx , attrs , item) ; }
};
}
