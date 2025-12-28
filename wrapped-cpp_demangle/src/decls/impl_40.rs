macro_rules! deps {
    () => {
        Result!();
        DemangleContext!();
        DemangleWrite!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < 'a , W > fmt :: Write for DemangleContext < 'a , W > where W : 'a + DemangleWrite , { fn write_str (& mut self , s : & str) -> fmt :: Result { if s . is_empty () { return Ok (()) ; } log ! ("DemangleContext::write: '{}'" , s) ; self . out . write_string (s) . map (| _ | { self . last_char_written = s . chars () . last () ; self . bytes_written += s . len () ; }) } }
    };
}

impl_40!()