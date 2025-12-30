// Generated macro for Attribute (struct)
macro_rules! Depcrate_read_unitAttribute {
() => {
// Module: crate::read::unit
// Provides: {"Attribute"}
// Dependencies: {}
# [doc = " An attribute in a `DebuggingInformationEntry`, consisting of a name and"] # [doc = " associated value."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct Attribute < R : Reader > { name : constants :: DwAt , value : AttributeValue < R > , }
};
}
