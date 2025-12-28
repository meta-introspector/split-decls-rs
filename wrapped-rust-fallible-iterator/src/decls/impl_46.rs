macro_rules! deps {
    () => {
        FoldStop!();
        ResultExt!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < T , E > ResultExt < T , E > for Result < T , FoldStop < T , E > > { # [inline] fn unpack_fold (self) -> Result < T , E > { match self { Ok (v) => Ok (v) , Err (FoldStop :: Break (v)) => Ok (v) , Err (FoldStop :: Err (e)) => Err (e) , } } }
    };
}

impl_46!()