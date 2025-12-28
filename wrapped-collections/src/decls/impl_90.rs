macro_rules! deps {
    () => {
        StockIterable!();
        IIterator_Impl!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < T > IIterator_Impl < T > for StockIterator_Impl < T > where T : RuntimeType , T :: Default : Clone , { fn Current (& self) -> Result < T > { let owner : & StockIterable < T > = & self . owner ; let current = self . current . load (std :: sync :: atomic :: Ordering :: Relaxed) ; if self . owner . values . len () > current { T :: from_default (& owner . values [current]) } else { Err (Error :: from (E_BOUNDS)) } } fn HasCurrent (& self) -> Result < bool > { let owner : & StockIterable < T > = & self . owner ; let current = self . current . load (std :: sync :: atomic :: Ordering :: Relaxed) ; Ok (owner . values . len () > current) } fn MoveNext (& self) -> Result < bool > { let owner : & StockIterable < T > = & self . owner ; let current = self . current . load (std :: sync :: atomic :: Ordering :: Relaxed) ; if current < owner . values . len () { self . current . fetch_add (1 , std :: sync :: atomic :: Ordering :: Relaxed) ; } Ok (owner . values . len () > current + 1) } fn GetMany (& self , values : & mut [T :: Default]) -> Result < u32 > { let owner : & StockIterable < T > = & self . owner ; let current = self . current . load (std :: sync :: atomic :: Ordering :: Relaxed) ; let actual = std :: cmp :: min (owner . values . len () - current , values . len ()) ; let (values , _) = values . split_at_mut (actual) ; values . clone_from_slice (& owner . values [current .. current + actual]) ; self . current . fetch_add (actual , std :: sync :: atomic :: Ordering :: Relaxed) ; Ok (actual as u32) } }
    };
}

impl_90!();