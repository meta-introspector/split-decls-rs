// Generated macro for impl_441 (impl)
macro_rules! Depcrate_filtersimpl_441 {
() => {
// Module: crate::filters
// Provides: {"impl_441"}
// Dependencies: {}
impl Filters { # [doc = " Adds a simple regex with a replacement."] pub (crate) fn add < S : Into < String > > (& mut self , regex : & str , replacement : S) { self . rules . push ((Regex :: new (regex) . expect ("invalid regex for snapshot filter rule") , replacement . into () ,)) ; } # [doc = " Clears all filters."] pub (crate) fn clear (& mut self) { self . rules . clear () ; } # [doc = " Applies all filters to the given snapshot."] pub (crate) fn apply_to < 's > (& self , s : & 's str) -> Cow < 's , str > { let mut rv = Cow :: Borrowed (s) ; for (regex , replacement) in & self . rules { match regex . replace_all (& rv , replacement) { Cow :: Borrowed (_) => continue , Cow :: Owned (value) => rv = Cow :: Owned (value) , } ; } rv } }
};
}
