macro_rules! deps {
    () => {
        OperationsIter!();
        OperationsIterInner!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl ExactSizeIterator for OperationsIter < '_ > { fn len (& self) -> usize { match & self . 0 { OperationsIterInner :: Single (opt) => usize :: from (opt . is_some ()) , OperationsIterInner :: Multiple (iter) => iter . len () , } } }
    };
}

impl_6!();