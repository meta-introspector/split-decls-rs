macro_rules! deps {
    () => {
        ParseAlphabetError!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl fmt :: Display for ParseAlphabetError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: InvalidLength => write ! (f , "Invalid length - must be 64 bytes") , Self :: DuplicatedByte (b) => write ! (f , "Duplicated byte: {:#04x}" , b) , Self :: UnprintableByte (b) => write ! (f , "Unprintable byte: {:#04x}" , b) , Self :: ReservedByte (b) => write ! (f , "Reserved byte: {:#04x}" , b) , } } }
    };
}

impl_208!();