macro_rules! deps {
    () => {
        LoopIdError!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl fmt :: Display for LoopIdError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { LoopIdError :: OutsideLoopScope => "not inside loop scope" , LoopIdError :: UnlabeledCfInWhileCondition => { "unlabeled control flow (break or continue) in while condition" } LoopIdError :: UnresolvedLabel => "label not found" , }) } }
    };
}

impl_234!();