macro_rules! deps {
    () => {
        TreeIterExt!();
        State!();
        Error!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl TreeIterExt for TreeRefIter < '_ > { fn traverse < StateMut , Find , V > (& self , state : StateMut , objects : Find , delegate : & mut V ,) -> Result < () , breadthfirst :: Error > where Find : gix_object :: Find , StateMut : BorrowMut < breadthfirst :: State > , V : gix_traverse :: tree :: Visit , { breadthfirst (* self , state , objects , delegate) } }
    };
}

impl_28!()