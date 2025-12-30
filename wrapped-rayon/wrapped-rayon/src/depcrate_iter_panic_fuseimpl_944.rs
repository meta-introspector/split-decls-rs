// Generated macro for impl_944 (impl)
macro_rules! Depcrate_iter_panic_fuseimpl_944 {
() => {
// Module: crate::iter::panic_fuse
// Provides: {"impl_944"}
// Dependencies: {}
impl < 'a , T , C > Folder < T > for PanicFuseFolder < 'a , C > where C : Folder < T > , { type Result = C :: Result ; fn consume (mut self , item : T) -> Self { self . base = self . base . consume (item) ; self } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = T > , { fn cool < 'a , T > (fuse : & 'a Fuse < '_ >) -> impl Fn (& T) -> bool + 'a { move | _ | ! fuse . panicked () } self . base = { let fuse = & self . fuse ; let iter = iter . into_iter () . take_while (cool (fuse)) ; self . base . consume_iter (iter) } ; self } fn complete (self) -> C :: Result { self . base . complete () } fn full (& self) -> bool { self . fuse . panicked () || self . base . full () } }
};
}
