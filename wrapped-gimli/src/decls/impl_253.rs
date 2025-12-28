macro_rules! deps {
    () => {
        Result!();
        Error!();
        Pointer!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        impl Pointer { # [inline] fn new (encoding : constants :: DwEhPe , address : u64) -> Pointer { if encoding . is_indirect () { Pointer :: Indirect (address) } else { Pointer :: Direct (address) } } # [doc = " Return the direct pointer value."] # [inline] pub fn direct (self) -> Result < u64 > { match self { Pointer :: Direct (p) => Ok (p) , Pointer :: Indirect (_) => Err (Error :: UnsupportedPointerEncoding) , } } # [doc = " Return the pointer value, discarding indirectness information."] # [inline] pub fn pointer (self) -> u64 { match self { Pointer :: Direct (p) | Pointer :: Indirect (p) => p , } } }
    };
}

impl_253!();