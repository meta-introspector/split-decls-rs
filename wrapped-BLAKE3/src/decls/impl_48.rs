macro_rules! deps {
    () => {
        HexErrorInner!();
        HexError!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl fmt :: Display for HexError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 { HexErrorInner :: InvalidByte (byte) => { if byte < 128 { write ! (f , "invalid hex character: {:?}" , byte as char) } else { write ! (f , "invalid hex character: 0x{:x}" , byte) } } HexErrorInner :: InvalidLen (len) => { write ! (f , "expected 64 hex bytes, received {}" , len) } } } }
    };
}

impl_48!()