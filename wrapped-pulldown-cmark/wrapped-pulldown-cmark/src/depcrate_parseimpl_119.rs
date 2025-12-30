// Generated macro for impl_119 (impl)
macro_rules! Depcrate_parseimpl_119 {
() => {
// Module: crate::parse
// Provides: {"impl_119"}
// Dependencies: {}
impl < 'input , 'b , 's > RefDefs < 'input > where 's : 'b , { # [doc = " Performs a lookup on reference label using unicode case folding."] pub fn get (& 's self , key : & 'b str) -> Option < & 'b LinkDef < 'input > > { self . 0 . get (& UniCase :: new (key . into ())) } # [doc = " Provides an iterator over all the document's reference definitions."] pub fn iter (& 's self) -> impl Iterator < Item = (& 's str , & 's LinkDef < 'input >) > { self . 0 . iter () . map (| (k , v) | (k . as_ref () , v)) } }
};
}
