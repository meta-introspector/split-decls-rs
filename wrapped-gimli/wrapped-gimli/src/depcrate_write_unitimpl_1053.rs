// Generated macro for impl_1053 (impl)
macro_rules! Depcrate_write_unitimpl_1053 {
() => {
// Module: crate::write::unit
// Provides: {"impl_1053"}
// Dependencies: {}
impl Attribute { # [doc = " Get the name of this attribute."] # [inline] pub fn name (& self) -> constants :: DwAt { self . name } # [doc = " Get the value of this attribute."] # [inline] pub fn get (& self) -> & AttributeValue { & self . value } # [doc = " Set the value of this attribute."] # [inline] pub fn set (& mut self , value : AttributeValue) { self . value = value ; } # [doc = " Return the type specification for this attribute."] fn specification (& self , encoding : Encoding) -> Result < AttributeSpecification > { Ok (AttributeSpecification :: new (self . name , self . value . form (encoding) ? ,)) } }
};
}
