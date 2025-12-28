macro_rules! deps {
    () => {
        OperationsIterInner!();
        DocumentOperations!();
        OperationsIter!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl DocumentOperations { # [doc = " Iterate over the operations of the document."] # [must_use] pub fn iter (& self) -> OperationsIter < '_ > { OperationsIter (match self { Self :: Single (op) => OperationsIterInner :: Single (Some (op)) , Self :: Multiple (ops) => OperationsIterInner :: Multiple (ops . iter ()) , }) } }
    };
}

impl_2!();