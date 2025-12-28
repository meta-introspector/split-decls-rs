macro_rules! deps {
    () => {
        Param!();
        TokenStream!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl Param { pub fn is_convertible (& self) -> bool { self . is_input () && self . ty . is_convertible () } pub fn is_input (& self) -> bool { ! self . def . flags () . contains (ParamAttributes :: Out) } pub fn is_optional (& self) -> bool { self . def . flags () . contains (ParamAttributes :: Optional) || self . def . has_attribute ("ReservedAttribute") } pub fn is_retval (& self) -> bool { if ! self . ty . is_pointer () { return false ; } if self . ty . is_void () { return false ; } let flags = self . def . flags () ; if flags . contains (ParamAttributes :: In) || ! flags . contains (ParamAttributes :: Out) || flags . contains (ParamAttributes :: Optional) { return false ; } for attribute in self . def . attributes () { if matches ! (attribute . name () , "NativeArrayInfoAttribute" | "MemorySizeAttribute") { return false ; } } if self . ty . deref () . size () > 16 { return false ; } true } pub fn write_ident (& self) -> TokenStream { to_ident (& self . def . name () . to_lowercase ()) } }
    };
}

impl_52!()