macro_rules! deps {
    () => {
        Binding!();
        ProgressState!();
        Progress!();
    };
}

macro_rules! impl_402 {
    () => {
        deps!();
        impl < 'a > Binding for Progress < 'a > { type Raw = * const raw :: git_indexer_progress ; unsafe fn from_raw (raw : * const raw :: git_indexer_progress) -> Progress < 'a > { Progress { raw : ProgressState :: Borrowed (raw) , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_indexer_progress { match self . raw { ProgressState :: Borrowed (raw) => raw , ProgressState :: Owned (ref raw) => raw as * const _ , } } }
    };
}

impl_402!()