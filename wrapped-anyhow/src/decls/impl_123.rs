macro_rules! deps {
    () => {
        Mut!();
        CastTo!();
        Own!();
        Ref!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < T > Own < T > where T : ? Sized , { pub fn new (ptr : Box < T >) -> Self { Own { ptr : unsafe { NonNull :: new_unchecked (Box :: into_raw (ptr)) } , } } pub fn cast < U : CastTo > (self) -> Own < U :: Target > { Own { ptr : self . ptr . cast () , } } pub unsafe fn boxed (self) -> Box < T > { unsafe { Box :: from_raw (self . ptr . as_ptr ()) } } pub fn by_ref (& self) -> Ref < T > { Ref { ptr : self . ptr , lifetime : PhantomData , } } pub fn by_mut (& mut self) -> Mut < T > { Mut { ptr : self . ptr , lifetime : PhantomData , } } }
    };
}

impl_123!()