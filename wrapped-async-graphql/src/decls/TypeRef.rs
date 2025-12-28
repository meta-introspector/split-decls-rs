macro_rules! TypeRef {
    () => {
        # [doc = " A type reference"] # [derive (Debug , Clone , Eq , PartialEq , Hash)] pub enum TypeRef { # [doc = " Named type"] Named (Cow < 'static , str >) , # [doc = " Non-null type"] NonNull (Box < TypeRef >) , # [doc = " List type"] List (Box < TypeRef >) , }
    };
}

TypeRef!()