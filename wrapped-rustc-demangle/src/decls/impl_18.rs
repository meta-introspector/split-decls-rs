macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < 's > fmt :: Display for Ident < 's > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . try_small_punycode_decode (| chars | { for & c in chars { c . fmt (f) ? ; } Ok (()) }) . unwrap_or_else (| | { if ! self . punycode . is_empty () { f . write_str ("punycode{") ? ; if ! self . ascii . is_empty () { f . write_str (self . ascii) ? ; f . write_str ("-") ? ; } f . write_str (self . punycode) ? ; f . write_str ("}") } else { f . write_str (self . ascii) } }) } }
    };
}

impl_18!();