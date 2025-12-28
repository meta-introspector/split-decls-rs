macro_rules! deps {
    () => {
        OperationDefinition!();
        Positioned!();
    };
}

macro_rules! OperationsIterInner {
    () => {
        deps!();
        # [derive (Debug , Clone)] enum OperationsIterInner < 'a > { Single (Option < & 'a Positioned < OperationDefinition > >) , Multiple (hash_map :: Iter < 'a , Name , Positioned < OperationDefinition > >) , }
    };
}

OperationsIterInner!()