macro_rules! deps {
    () => {
        StorageDeadOrDrop!();
    };
}

macro_rules! WriteKind {
    () => {
        deps!();
        # [doc = " Kind of write access to a value"] # [doc = " (For informational purposes only)"] # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum WriteKind { StorageDeadOrDrop , Replace , MutableBorrow (BorrowKind) , Mutate , Move , }
    };
}

WriteKind!()