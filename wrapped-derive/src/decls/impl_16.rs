macro_rules! deps {
    () => {
        Resolvability!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl FromMeta for Resolvability { fn from_word () -> darling :: Result < Self > { Ok (Resolvability :: Unresolvable { key : None }) } fn from_value (value : & Lit) -> darling :: Result < Self > { match value { Lit :: Bool (LitBool { value : true , .. }) => Ok (Resolvability :: Unresolvable { key : None }) , Lit :: Bool (LitBool { value : false , .. }) => Ok (Resolvability :: Resolvable) , Lit :: Str (str) => Ok (Resolvability :: Unresolvable { key : Some (str . value ()) , }) , _ => Err (darling :: Error :: unexpected_lit_type (value)) , } } }
    };
}

impl_16!();