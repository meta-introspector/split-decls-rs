macro_rules! deps {
    () => {
        CastTo!();
        Mut!();
        Ref!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < 'a , T > Mut < 'a , T > where T : ? Sized , { # [cfg (anyhow_no_ptr_addr_of)] pub fn new (ptr : & 'a mut T) -> Self { Mut { ptr : NonNull :: from (ptr) , lifetime : PhantomData , } } pub fn cast < U : CastTo > (self) -> Mut < 'a , U :: Target > { Mut { ptr : self . ptr . cast () , lifetime : PhantomData , } } # [cfg (not (anyhow_no_ptr_addr_of))] pub fn by_ref (self) -> Ref < 'a , T > { Ref { ptr : self . ptr , lifetime : PhantomData , } } pub fn extend < 'b > (self) -> Mut < 'b , T > { Mut { ptr : self . ptr , lifetime : PhantomData , } } pub unsafe fn deref_mut (self) -> & 'a mut T { unsafe { & mut * self . ptr . as_ptr () } } }
    };
}

impl_131!();