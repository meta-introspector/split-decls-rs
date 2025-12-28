macro_rules! deps {
    () => {
        Positioned!();
        OperationDefinition!();
    };
}

macro_rules! OperationsIterInner {
    () => {
        deps!();
        # [derive (Debug , Clone)] enum OperationsIterInner < 'a > { Single (Option < & 'a Positioned < OperationDefinition > >) , Multiple (hash_map :: Iter < 'a , Name , Positioned < OperationDefinition > >) , }
    };
}

OperationsIterInner!();