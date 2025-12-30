// Generated macro for impl_596 (impl)
macro_rules! Depcrate_iter_filter_mapimpl_596 {
() => {
// Module: crate::iter::filter_map
// Provides: {"impl_596"}
// Dependencies: {}
impl < 'p , T , U , C , P > Folder < T > for FilterMapFolder < 'p , C , P > where C : Folder < U > , P : Fn (T) -> Option < U > + Sync + 'p , { type Result = C :: Result ; fn consume (self , item : T) -> Self { let filter_op = self . filter_op ; if let Some (mapped_item) = filter_op (item) { let base = self . base . consume (mapped_item) ; FilterMapFolder { base , filter_op } } else { self } } fn complete (self) -> C :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
};
}
