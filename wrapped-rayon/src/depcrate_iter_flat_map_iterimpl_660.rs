// Generated macro for impl_660 (impl)
macro_rules! Depcrate_iter_flat_map_iterimpl_660 {
() => {
// Module: crate::iter::flat_map_iter
// Provides: {"impl_660"}
// Dependencies: {}
impl < 'f , T , U , C , F > Folder < T > for FlatMapIterFolder < 'f , C , F > where C : Folder < U :: Item > , F : Fn (T) -> U , U : IntoIterator , { type Result = C :: Result ; fn consume (self , item : T) -> Self { let map_op = self . map_op ; let base = self . base . consume_iter (map_op (item)) ; FlatMapIterFolder { base , map_op } } fn consume_iter < I > (self , iter : I) -> Self where I : IntoIterator < Item = T > , { let map_op = self . map_op ; let iter = iter . into_iter () . flat_map (map_op) ; let base = self . base . consume_iter (iter) ; FlatMapIterFolder { base , map_op } } fn complete (self) -> Self :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
};
}
