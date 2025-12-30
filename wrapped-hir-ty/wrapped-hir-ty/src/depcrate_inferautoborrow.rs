// Generated macro for AutoBorrow (enum)
macro_rules! Depcrate_inferAutoBorrow {
() => {
// Module: crate::infer
// Provides: {"AutoBorrow"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum AutoBorrow { # [doc = " Converts from T to &T."] Ref (AutoBorrowMutability) , # [doc = " Converts from T to *T."] RawPtr (Mutability) , }
};
}
