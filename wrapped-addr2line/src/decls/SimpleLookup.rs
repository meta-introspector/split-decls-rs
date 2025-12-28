macro_rules! SimpleLookup {
    () => {
        pub (crate) struct SimpleLookup < T , R , F > where F : FnOnce (Option < Arc < gimli :: Dwarf < R > > >) -> T , R : gimli :: Reader , { f : F , phantom : PhantomData < (T , R) > , }
    };
}

SimpleLookup!()