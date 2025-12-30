// Generated macro for impl_152 (impl)
macro_rules! Depcrate_file_section_bodyimpl_152 {
() => {
// Module: crate::file::section::body
// Provides: {"impl_152"}
// Dependencies: {}
impl < 'event > Iterator for BodyIter < 'event > { type Item = (ValueName < 'event > , Cow < 'event , BStr >) ; fn next (& mut self) -> Option < Self :: Item > { let mut key = None ; let mut partial_value = BString :: default () ; let mut value = None ; for event in self . 0 . by_ref () { match event { Event :: SectionValueName (k) => key = Some (k) , Event :: Value (v) => { value = Some (v) ; break ; } Event :: ValueNotDone (v) => partial_value . push_str (v . as_ref ()) , Event :: ValueDone (v) => { partial_value . push_str (v . as_ref ()) ; value = Some (partial_value . into ()) ; break ; } _ => () , } } key . zip (value . map (normalize)) } }
};
}
