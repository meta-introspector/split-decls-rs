macro_rules! deps {
    () => {
        ChallengeRef!();
        ParamsPrinter!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ChallengeRef < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("ChallengeRef") . field ("scheme" , & self . scheme) . field ("params" , & ParamsPrinter (& self . params)) . finish () } }
    };
}

impl_63!()