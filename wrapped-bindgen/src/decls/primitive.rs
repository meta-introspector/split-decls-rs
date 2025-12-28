macro_rules! deps {
    () => {
        TokenStream!();
        ToTokens!();
    };
}

macro_rules! primitive {
    () => {
        deps!();
        macro_rules ! primitive { ($ ($ t : ident => $ name : ident) *) => ($ (impl ToTokens for $ t { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . push_space () ; tokens . push_str (& self . to_string ()) ; tokens . push_str (stringify ! ($ t)) ; } }) *) }
    };
}

primitive!();