// Generated macro for if_enum_attrs_present (function)
macro_rules! Depcrate_errorif_enum_attrs_present {
() => {
// Module: crate::error
// Provides: {"if_enum_attrs_present"}
// Dependencies: {}
# [doc = " Ensures that things only allowed on an enum variant is not present on"] # [doc = " `item` which is not an enum variant."] pub fn if_enum_attrs_present (ctx : Ctx , attrs : & ParsedAttributes , item : & str) { if_skip_present (ctx , attrs , item) ; if_weight_present (ctx , attrs , item) ; }
};
}
