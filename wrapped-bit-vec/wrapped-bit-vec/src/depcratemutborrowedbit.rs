// Generated macro for MutBorrowedBit (struct)
macro_rules! DepcrateMutBorrowedBit {
() => {
// Module: crate
// Provides: {"MutBorrowedBit"}
// Dependencies: {}
# [derive (Debug)] pub struct MutBorrowedBit < 'a , B : 'a + BitBlock > { vec : Rc < RefCell < & 'a mut BitVec < B > > > , index : usize , # [cfg (debug_assertions)] old_value : bool , new_value : bool , }
};
}
