macro_rules! deps {
    () => {
        InferCtxtUndoLogs!();
    };
}

macro_rules! UnificationTable {
    () => {
        deps!();
        pub (crate) type UnificationTable < 'a , 'tcx , T > = ut :: UnificationTable < ut :: InPlace < T , & 'a mut ut :: UnificationStorage < T > , & 'a mut InferCtxtUndoLogs < 'tcx > > , > ;
    };
}

UnificationTable!();