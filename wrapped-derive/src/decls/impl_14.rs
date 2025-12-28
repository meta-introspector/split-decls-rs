macro_rules! deps {
    () => {
        Deprecation!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl FromMeta for Deprecation { fn from_word () -> darling :: Result < Self > { Ok (Deprecation :: Deprecated { reason : None }) } fn from_value (value : & Lit) -> darling :: Result < Self > { match value { Lit :: Bool (LitBool { value : true , .. }) => Ok (Deprecation :: Deprecated { reason : None }) , Lit :: Bool (LitBool { value : false , .. }) => Ok (Deprecation :: NoDeprecated) , Lit :: Str (str) => Ok (Deprecation :: Deprecated { reason : Some (str . value ()) , }) , _ => Err (darling :: Error :: unexpected_lit_type (value)) , } } }
    };
}

impl_14!();