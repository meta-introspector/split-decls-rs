// Generated macro for impl_214 (impl)
macro_rules! Depcrate_packed_testsimpl_214 {
() => {
// Module: crate::packed::tests
// Provides: {"impl_214"}
// Dependencies: {}
impl SearchTest { fn variations (& self) -> Vec < SearchTestOwned > { let count = if cfg ! (miri) { 1 } else { 261 } ; let mut tests = vec ! [] ; for i in 0 .. count { tests . push (self . offset_prefix (i)) ; tests . push (self . offset_suffix (i)) ; tests . push (self . offset_both (i)) ; } tests } fn offset_both (& self , off : usize) -> SearchTestOwned { SearchTestOwned { offset : off , name : self . name . to_string () , patterns : self . patterns . iter () . map (| s | s . to_string ()) . collect () , haystack : format ! ("{}{}{}" , "Z" . repeat (off) , self . haystack , "Z" . repeat (off)) , matches : self . matches . iter () . map (| & (id , s , e) | (id , s + off , e + off)) . collect () , } } fn offset_prefix (& self , off : usize) -> SearchTestOwned { SearchTestOwned { offset : off , name : self . name . to_string () , patterns : self . patterns . iter () . map (| s | s . to_string ()) . collect () , haystack : format ! ("{}{}" , "Z" . repeat (off) , self . haystack) , matches : self . matches . iter () . map (| & (id , s , e) | (id , s + off , e + off)) . collect () , } } fn offset_suffix (& self , off : usize) -> SearchTestOwned { SearchTestOwned { offset : off , name : self . name . to_string () , patterns : self . patterns . iter () . map (| s | s . to_string ()) . collect () , haystack : format ! ("{}{}" , self . haystack , "Z" . repeat (off)) , matches : self . matches . to_vec () , } } }
};
}
