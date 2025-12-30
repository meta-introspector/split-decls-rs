// Generated macro for impl_176 (impl)
macro_rules! Depcrate_signaturesimpl_176 {
() => {
// Module: crate::signatures
// Provides: {"impl_176"}
// Dependencies: {}
impl VariantFields { pub fn len (& self) -> usize { self . fields . len () } pub fn fields (& self) -> & Arena < FieldData > { & self . fields } pub fn field (& self , name : & Name) -> Option < LocalFieldId > { self . fields () . iter () . find_map (| (id , data) | if & data . name == name { Some (id) } else { None }) } }
};
}
