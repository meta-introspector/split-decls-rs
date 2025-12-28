macro_rules! deps {
    () => {
        FnTrait!();
    };
}

macro_rules! impl_725 {
    () => {
        deps!();
        impl fmt :: Display for FnTrait { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { FnTrait :: FnOnce => write ! (f , "FnOnce") , FnTrait :: FnMut => write ! (f , "FnMut") , FnTrait :: Fn => write ! (f , "Fn") , FnTrait :: AsyncFnOnce => write ! (f , "AsyncFnOnce") , FnTrait :: AsyncFnMut => write ! (f , "AsyncFnMut") , FnTrait :: AsyncFn => write ! (f , "AsyncFn") , } } }
    };
}

impl_725!()