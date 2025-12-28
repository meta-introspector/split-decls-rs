macro_rules! deps {
    () => {
        InputSetterWithDurability!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl ToTokens for InputSetterWithDurability { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let sig = & mut self . signature . clone () ; let ty = & self . return_type ; let fn_ident = & sig . ident ; let setter_ident = format_ident ! ("set_{}" , fn_ident) ; let create_data_ident = & self . create_data_ident ; sig . ident = format_ident ! ("set_{}_with_durability" , fn_ident) ; let value_argument : PatType = parse_quote ! (__value : # ty) ; sig . inputs . push (FnArg :: Typed (value_argument . clone ())) ; let durability_argument : PatType = parse_quote ! (durability : salsa :: Durability) ; sig . inputs . push (FnArg :: Typed (durability_argument . clone ())) ; let mut_receiver : Receiver = parse_quote ! (& mut self) ; if let Some (og) = sig . inputs . first_mut () { * og = FnArg :: Receiver (mut_receiver) } sig . output = ReturnType :: Default ; let value = & value_argument . pat ; let durability = & durability_argument . pat ; let method = quote ! { # sig { use salsa :: Setter ; let data = # create_data_ident (self) ; data .# setter_ident (self) . with_durability (# durability) . to (Some (# value)) ; } } ; method . to_tokens (tokens) ; } }
    };
}

impl_8!()