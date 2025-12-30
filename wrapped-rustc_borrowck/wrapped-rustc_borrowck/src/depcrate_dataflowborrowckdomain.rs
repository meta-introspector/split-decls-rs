// Generated macro for BorrowckDomain (struct)
macro_rules! Depcrate_dataflowBorrowckDomain {
() => {
// Module: crate::dataflow
// Provides: {"BorrowckDomain"}
// Dependencies: {}
# [doc = " The transient state of the dataflow analyses used by the borrow checker."] # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) struct BorrowckDomain { pub (crate) borrows : BorrowsDomain , pub (crate) uninits : MaybeUninitializedPlacesDomain , pub (crate) ever_inits : EverInitializedPlacesDomain , }
};
}
