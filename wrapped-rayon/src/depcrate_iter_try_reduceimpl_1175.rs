// Generated macro for impl_1175 (impl)
macro_rules! Depcrate_iter_try_reduceimpl_1175 {
() => {
// Module: crate::iter::try_reduce
// Provides: {"impl_1175"}
// Dependencies: {}
impl < 'r , R , T > Folder < T > for TryReduceFolder < 'r , R , T > where R : Fn (T :: Output , T :: Output) -> T , T : Try , { type Result = T ; fn consume (mut self , item : T) -> Self { let reduce_op = self . reduce_op ; self . control = match (self . control , item . branch ()) { (Continue (left) , Continue (right)) => reduce_op (left , right) . branch () , (control @ Break (_) , _) | (_ , control @ Break (_)) => control , } ; if let Break (_) = self . control { self . full . store (true , Ordering :: Relaxed) ; } self } fn complete (self) -> T { match self . control { Continue (c) => T :: from_output (c) , Break (r) => T :: from_residual (r) , } } fn full (& self) -> bool { match self . control { Break (_) => true , _ => self . full . load (Ordering :: Relaxed) , } } }
};
}
