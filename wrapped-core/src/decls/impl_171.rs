macro_rules! deps {
    () => {
        OutRef!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < T : Type < T > > OutRef < '_ , T > { # [doc = " Returns `true` if the argument is null."] pub fn is_null (& self) -> bool { self . 0 . is_null () } # [doc = " Overwrites a memory location with the given value without reading or dropping the old value."] pub fn write (self , value : T :: Default) -> Result < () > { if self . 0 . is_null () { Err (Error :: from_hresult (imp :: E_POINTER)) } else { unsafe { * self . 0 = core :: mem :: transmute_copy (& value) } core :: mem :: forget (value) ; Ok (()) } } }
    };
}

impl_171!();