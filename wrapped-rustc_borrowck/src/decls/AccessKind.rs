macro_rules! AccessKind {
    () => {
        # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub (crate) enum AccessKind { MutableBorrow , Mutate , }
    };
}

AccessKind!()