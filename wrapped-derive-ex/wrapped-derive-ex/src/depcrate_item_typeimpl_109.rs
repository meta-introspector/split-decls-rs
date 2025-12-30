// Generated macro for impl_109 (impl)
macro_rules! Depcrate_item_typeimpl_109 {
() => {
// Module: crate::item_type
// Provides: {"impl_109"}
// Dependencies: {}
impl HelperAttributeForDebug { fn from_attrs (attrs : & [Attribute]) -> Result < Self > { if let Some (args) = parse_single :: < ArgsForDebug > (attrs , "debug") ? { Ok (Self { transparent : args . transparent , skip : if args . skip . value () || args . ignore . value () { Flag { span : args . skip . span . or (args . ignore . span) } } else { Flag :: NONE } , bounds : Bounds :: from (& args . bound) , }) } else { Ok (Self :: default ()) } } }
};
}
