macro_rules! deps {
    () => {
        Display!();
        Trait!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl ToTokens for Trait { fn to_tokens (& self , tokens : & mut TokenStream) { let trait_name = match self { Trait :: Debug => "Debug" , Trait :: Display => "Display" , Trait :: Octal => "Octal" , Trait :: LowerHex => "LowerHex" , Trait :: UpperHex => "UpperHex" , Trait :: Pointer => "Pointer" , Trait :: Binary => "Binary" , Trait :: LowerExp => "LowerExp" , Trait :: UpperExp => "UpperExp" , } ; let ident = Ident :: new (trait_name , Span :: call_site ()) ; tokens . extend (quote ! (:: core :: fmt ::# ident)) ; } }
    };
}

impl_26!();