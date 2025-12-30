// Generated macro for impl_413 (impl)
macro_rules! Depcrate_memmemimpl_413 {
() => {
// Module: crate::memmem
// Provides: {"impl_413"}
// Dependencies: {}
impl < 'h , 'n > Iterator for FindRevIter < 'h , 'n > { type Item = usize ; fn next (& mut self) -> Option < usize > { let pos = match self . pos { None => return None , Some (pos) => pos , } ; let result = self . finder . rfind (& self . haystack [.. pos]) ; match result { None => None , Some (i) => { if pos == i { self . pos = pos . checked_sub (1) ; } else { self . pos = Some (i) ; } Some (i) } } } }
};
}
