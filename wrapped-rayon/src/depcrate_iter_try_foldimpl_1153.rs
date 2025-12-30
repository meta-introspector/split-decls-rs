// Generated macro for impl_1153 (impl)
macro_rules! Depcrate_iter_try_foldimpl_1153 {
() => {
// Module: crate::iter::try_fold
// Provides: {"impl_1153"}
// Dependencies: {}
impl < 'r , C , U , F , T > Folder < T > for TryFoldFolder < 'r , C , U , F > where C : Folder < U > , F : Fn (U :: Output , T) -> U + Sync , U : Try , { type Result = C :: Result ; fn consume (mut self , item : T) -> Self { let fold_op = self . fold_op ; if let Continue (acc) = self . control { self . control = fold_op (acc , item) . branch () ; } self } fn complete (self) -> C :: Result { let item = match self . control { Continue (c) => U :: from_output (c) , Break (r) => U :: from_residual (r) , } ; self . base . consume (item) . complete () } fn full (& self) -> bool { match self . control { Break (_) => true , _ => self . base . full () , } } }
};
}
