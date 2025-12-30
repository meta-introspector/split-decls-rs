// Generated macro for impl_44 (impl)
macro_rules! Depcrate_sourceimpl_44 {
() => {
// Module: crate::source
// Provides: {"impl_44"}
// Dependencies: {}
impl Files { pub fn push (& mut self , name : & str , contents : & [u8]) { match self . files . entry (name . to_owned ()) { Entry :: Vacant (entry) => { entry . insert (contents . to_owned ()) ; } Entry :: Occupied (ref mut entry) => { entry . get_mut () . extend_from_slice (contents) ; } } } pub fn get_size (& mut self , name : & str) -> Option < usize > { self . files . get (name) . map (| data | data . len ()) } pub fn remove (& mut self , name : & str) -> Option < Vec < u8 > > { self . files . remove (name) } pub fn iter (& self) -> impl Iterator < Item = (& '_ str , & '_ [u8]) > { self . files . iter () . map (| p | (p . 0 . as_str () , p . 1 . as_slice ())) } }
};
}
