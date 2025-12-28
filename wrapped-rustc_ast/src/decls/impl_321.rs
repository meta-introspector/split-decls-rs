macro_rules! deps {
    () => {
        Const!();
        DiffActivity!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl Display for DiffActivity { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { match self { DiffActivity :: None => write ! (f , "None") , DiffActivity :: Const => write ! (f , "Const") , DiffActivity :: Active => write ! (f , "Active") , DiffActivity :: ActiveOnly => write ! (f , "ActiveOnly") , DiffActivity :: Dual => write ! (f , "Dual") , DiffActivity :: Dualv => write ! (f , "Dualv") , DiffActivity :: DualOnly => write ! (f , "DualOnly") , DiffActivity :: DualvOnly => write ! (f , "DualvOnly") , DiffActivity :: Duplicated => write ! (f , "Duplicated") , DiffActivity :: DuplicatedOnly => write ! (f , "DuplicatedOnly") , DiffActivity :: FakeActivitySize (s) => write ! (f , "FakeActivitySize({:?})" , s) , } } }
    };
}

impl_321!();