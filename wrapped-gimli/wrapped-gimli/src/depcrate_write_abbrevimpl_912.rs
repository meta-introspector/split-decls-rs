// Generated macro for impl_912 (impl)
macro_rules! Depcrate_write_abbrevimpl_912 {
() => {
// Module: crate::write::abbrev
// Provides: {"impl_912"}
// Dependencies: {}
impl AttributeSpecification { # [doc = " Construct a new `AttributeSpecification`."] # [inline] pub fn new (name : constants :: DwAt , form : constants :: DwForm) -> AttributeSpecification { AttributeSpecification { name , form } } # [doc = " Write the attribute specification to the `.debug_abbrev` section."] # [inline] pub fn write < W : Writer > (& self , w : & mut DebugAbbrev < W >) -> Result < () > { w . write_uleb128 (self . name . 0 . into ()) ? ; w . write_uleb128 (self . form . 0 . into ()) } }
};
}
