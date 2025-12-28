macro_rules! AllocatorKind {
    () => {
        # [derive (Clone , Debug , Copy , Eq , PartialEq , HashStable_Generic)] pub enum AllocatorKind { Global , Default , }
    };
}

AllocatorKind!()