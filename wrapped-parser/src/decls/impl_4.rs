macro_rules! deps {
    () => {
        OperationsIter!();
        OperationsIterInner!();
        OperationDefinition!();
        Positioned!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 'a > Iterator for OperationsIter < 'a > { type Item = (Option < & 'a Name > , & 'a Positioned < OperationDefinition >) ; fn next (& mut self) -> Option < Self :: Item > { match & mut self . 0 { OperationsIterInner :: Single (op) => op . take () . map (| op | (None , op)) , OperationsIterInner :: Multiple (iter) => iter . next () . map (| (name , op) | (Some (name) , op)) , } } fn size_hint (& self) -> (usize , Option < usize >) { let size = self . len () ; (size , Some (size)) } }
    };
}

impl_4!()