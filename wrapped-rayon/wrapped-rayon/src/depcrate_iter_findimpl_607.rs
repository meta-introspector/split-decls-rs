// Generated macro for impl_607 (impl)
macro_rules! Depcrate_iter_findimpl_607 {
() => {
// Module: crate::iter::find
// Provides: {"impl_607"}
// Dependencies: {}
impl < 'p , T , P > Folder < T > for FindFolder < 'p , T , P > where P : Fn (& T) -> bool + 'p , { type Result = Option < T > ; fn consume (mut self , item : T) -> Self { if (self . find_op) (& item) { self . found . store (true , Ordering :: Relaxed) ; self . item = Some (item) ; } self } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = T > , { fn not_full < T > (found : & AtomicBool) -> impl Fn (& T) -> bool + '_ { move | _ | ! found . load (Ordering :: Relaxed) } self . item = iter . into_iter () . take_while (not_full (self . found)) . find (self . find_op) ; if self . item . is_some () { self . found . store (true , Ordering :: Relaxed) } self } fn complete (self) -> Self :: Result { self . item } fn full (& self) -> bool { self . found . load (Ordering :: Relaxed) } }
};
}
