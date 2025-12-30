// Generated macro for MutBorrowKind (enum)
macro_rules! Depcrate_mirMutBorrowKind {
() => {
// Module: crate::mir
// Provides: {"MutBorrowKind"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Clone , Copy , PartialOrd , Ord)] pub enum MutBorrowKind { # [doc = " Data must be immutable but not aliasable. This kind of borrow cannot currently"] # [doc = " be expressed by the user and is used only in implicit closure bindings."] ClosureCapture , Default , # [doc = " This borrow arose from method-call auto-ref"] # [doc = " (i.e., adjustment::Adjust::Borrow)."] TwoPhasedBorrow , }
};
}
