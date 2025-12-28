macro_rules! deps {
    () => {
        HybridProtection!();
        RefCnt!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < T : RefCnt > Protected < T > for HybridProtection < T > { # [inline] fn from_inner (ptr : T) -> Self { Self { debt : None , ptr : ManuallyDrop :: new (ptr) , } } # [inline] fn into_inner (mut self) -> T { match self . debt . take () { None => () , Some (debt) => { let ptr = T :: inc (& self . ptr) ; if ! debt . pay :: < T > (ptr) { unsafe { T :: dec (ptr) } ; } } } let inner = unsafe { ptr :: read (self . ptr . deref ()) } ; mem :: forget (self) ; inner } }
    };
}

impl_107!();