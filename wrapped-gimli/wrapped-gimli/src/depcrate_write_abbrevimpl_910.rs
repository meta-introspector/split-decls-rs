// Generated macro for impl_910 (impl)
macro_rules! Depcrate_write_abbrevimpl_910 {
() => {
// Module: crate::write::abbrev
// Provides: {"impl_910"}
// Dependencies: {}
impl Abbreviation { # [doc = " Construct a new `Abbreviation`."] # [inline] pub fn new (tag : constants :: DwTag , has_children : bool , attributes : Vec < AttributeSpecification > ,) -> Abbreviation { Abbreviation { tag , has_children , attributes , } } # [doc = " Write the abbreviation to the `.debug_abbrev` section."] pub fn write < W : Writer > (& self , w : & mut DebugAbbrev < W >) -> Result < () > { w . write_uleb128 (self . tag . 0 . into ()) ? ; w . write_u8 (if self . has_children { constants :: DW_CHILDREN_yes . 0 } else { constants :: DW_CHILDREN_no . 0 }) ? ; for attr in & self . attributes { attr . write (w) ? ; } w . write_u8 (0) ? ; w . write_u8 (0) } }
};
}
