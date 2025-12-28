macro_rules! deps {
    () => {
        BorrowsDomain!();
    };
}

macro_rules! BorrowckDomain {
    () => {
        deps!();
        # [doc = " The transient state of the dataflow analyses used by the borrow checker."] # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) struct BorrowckDomain { pub (crate) borrows : BorrowsDomain , pub (crate) uninits : MaybeUninitializedPlacesDomain , pub (crate) ever_inits : EverInitializedPlacesDomain , }
    };
}

BorrowckDomain!();