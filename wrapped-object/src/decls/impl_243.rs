macro_rules! deps {
    () => {
        ImageSymbol!();
        CoffHeader!();
        ReadRef!();
        CoffSymbol!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > CoffSymbol < 'data , 'file , R , Coff > { # [inline] # [doc = " Get the raw `ImageSymbol` struct."] # [deprecated (note = "Use `coff_symbol` instead")] pub fn raw_symbol (& self) -> & 'data Coff :: ImageSymbol { self . symbol } # [doc = " Get the raw `ImageSymbol` struct."] pub fn coff_symbol (& self) -> & 'data Coff :: ImageSymbol { self . symbol } }
    };
}

impl_243!();