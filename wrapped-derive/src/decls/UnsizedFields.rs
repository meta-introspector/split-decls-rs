macro_rules! deps {
    () => {
        UnsizedField!();
    };
}

macro_rules! UnsizedFields {
    () => {
        deps!();
        struct UnsizedFields < 'a > { fields : Vec < UnsizedField < 'a > > , format_param : TokenStream2 , }
    };
}

UnsizedFields!();