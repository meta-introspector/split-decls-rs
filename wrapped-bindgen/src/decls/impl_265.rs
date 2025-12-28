macro_rules! deps {
    () => {
        Signature!();
        Config!();
        Type!();
        TokenStream!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl Config < '_ > { pub fn write_return_sig (& self , method : MethodDef , signature : & Signature , underlying_types : bool ,) -> TokenStream { match & signature . return_type { Type :: Void => { if method . has_attribute ("DoesNotReturnAttribute") { quote ! { -> ! } } else { quote ! { } } } ty => { let ty = if underlying_types { ty . underlying_type () . write_default (self) } else { ty . write_default (self) } ; quote ! { -> # ty } } } } }
    };
}

impl_265!()