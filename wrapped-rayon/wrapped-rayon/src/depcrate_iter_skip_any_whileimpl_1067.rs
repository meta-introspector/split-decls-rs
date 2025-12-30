// Generated macro for impl_1067 (impl)
macro_rules! Depcrate_iter_skip_any_whileimpl_1067 {
() => {
// Module: crate::iter::skip_any_while
// Provides: {"impl_1067"}
// Dependencies: {}
impl < 'p , T , C , P > Folder < T > for SkipAnyWhileFolder < 'p , C , P > where C : Folder < T > , P : Fn (& T) -> bool + 'p , { type Result = C :: Result ; fn consume (mut self , item : T) -> Self { if ! skip (& item , self . skipping , self . predicate) { self . base = self . base . consume (item) ; } self } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = T > , { self . base = self . base . consume_iter (iter . into_iter () . skip_while (move | x | skip (x , self . skipping , self . predicate)) ,) ; self } fn complete (self) -> C :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
};
}
