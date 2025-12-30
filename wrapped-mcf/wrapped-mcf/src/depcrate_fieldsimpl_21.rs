// Generated macro for impl_21 (impl)
macro_rules! Depcrate_fieldsimpl_21 {
() => {
// Module: crate::fields
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a > Iterator for Fields < 'a > { type Item = Field < 'a > ; fn next (& mut self) -> Option < Field < 'a > > { if self . 0 . is_empty () { return None ; } match self . 0 . split_once (DELIMITER) { Some ((field , rest)) => { self . 0 = rest ; Some (Field (field)) } None => { let ret = self . 0 ; self . 0 = "" ; Some (Field (ret)) } } } }
};
}
