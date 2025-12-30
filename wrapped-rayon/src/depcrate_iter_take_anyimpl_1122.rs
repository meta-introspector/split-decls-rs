// Generated macro for impl_1122 (impl)
macro_rules! Depcrate_iter_take_anyimpl_1122 {
() => {
// Module: crate::iter::take_any
// Provides: {"impl_1122"}
// Dependencies: {}
impl < 'f , T , C > Folder < T > for TakeAnyFolder < 'f , C > where C : Folder < T > , { type Result = C :: Result ; fn consume (mut self , item : T) -> Self { if checked_decrement (self . count) { self . base = self . base . consume (item) ; } self } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = T > , { self . base = self . base . consume_iter (iter . into_iter () . take_while (move | _ | checked_decrement (self . count)) ,) ; self } fn complete (self) -> C :: Result { self . base . complete () } fn full (& self) -> bool { self . count . load (Ordering :: Relaxed) == 0 || self . base . full () } }
};
}
