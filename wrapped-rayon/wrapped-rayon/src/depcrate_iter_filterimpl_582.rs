// Generated macro for impl_582 (impl)
macro_rules! Depcrate_iter_filterimpl_582 {
() => {
// Module: crate::iter::filter
// Provides: {"impl_582"}
// Dependencies: {}
impl < 'p , C , P , T > Folder < T > for FilterFolder < 'p , C , P > where C : Folder < T > , P : Fn (& T) -> bool + 'p , { type Result = C :: Result ; fn consume (self , item : T) -> Self { let filter_op = self . filter_op ; if filter_op (& item) { let base = self . base . consume (item) ; FilterFolder { base , filter_op } } else { self } } fn complete (self) -> Self :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
};
}
