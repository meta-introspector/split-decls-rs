macro_rules! deps {
    () => {
        IIterator_Impl!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < T > IIterator_Impl < T > for StockVectorViewIterator_Impl < T > where T : RuntimeType , T :: Default : Clone + PartialEq , { fn Current (& self) -> Result < T > { let current = self . current . load (std :: sync :: atomic :: Ordering :: Relaxed) ; if let Some (item) = self . owner . values . get (current) { T :: from_default (item) } else { Err (Error :: from (E_BOUNDS)) } } fn HasCurrent (& self) -> Result < bool > { let current = self . current . load (std :: sync :: atomic :: Ordering :: Relaxed) ; Ok (self . owner . values . len () > current) } fn MoveNext (& self) -> Result < bool > { let current = self . current . load (std :: sync :: atomic :: Ordering :: Relaxed) ; if current < self . owner . values . len () { self . current . fetch_add (1 , std :: sync :: atomic :: Ordering :: Relaxed) ; } Ok (self . owner . values . len () > current + 1) } fn GetMany (& self , values : & mut [T :: Default]) -> Result < u32 > { let current = self . current . load (std :: sync :: atomic :: Ordering :: Relaxed) ; let actual = std :: cmp :: min (self . owner . values . len () - current , values . len ()) ; let (values , _) = values . split_at_mut (actual) ; values . clone_from_slice (& self . owner . values [current .. current + actual]) ; self . current . fetch_add (actual , std :: sync :: atomic :: Ordering :: Relaxed) ; Ok (actual as u32) } }
    };
}

impl_106!()