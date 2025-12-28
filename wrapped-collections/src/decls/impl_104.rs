macro_rules! deps {
    () => {
        IVectorView_Impl!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < T > IVectorView_Impl < T > for StockVectorView_Impl < T > where T : RuntimeType , T :: Default : Clone + PartialEq , { fn GetAt (& self , index : u32) -> Result < T > { let item = self . values . get (index as usize) . ok_or_else (| | Error :: from (E_BOUNDS)) ? ; T :: from_default (item) } fn Size (& self) -> Result < u32 > { Ok (self . values . len () . try_into () ?) } fn IndexOf (& self , value : Ref < T > , result : & mut u32) -> Result < bool > { match self . values . iter () . position (| element | element == & * value) { Some (index) => { * result = index as u32 ; Ok (true) } None => Ok (false) , } } fn GetMany (& self , current : u32 , values : & mut [T :: Default]) -> Result < u32 > { let current = current as usize ; if current >= self . values . len () { return Ok (0) ; } let actual = std :: cmp :: min (self . values . len () - current , values . len ()) ; let (values , _) = values . split_at_mut (actual) ; values . clone_from_slice (& self . values [current .. current + actual]) ; Ok (actual as u32) } }
    };
}

impl_104!();