macro_rules! deps {
    () => {
        SingleOrMultiIndex!();
        IndexId!();
    };
}

macro_rules! IndexLookup {
    () => {
        deps!();
        pub struct IndexLookup { pub (crate) file : SingleOrMultiIndex , # [doc = " The index we were found at in the slot map"] pub (crate) id : types :: IndexId , }
    };
}

IndexLookup!();