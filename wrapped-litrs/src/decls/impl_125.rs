macro_rules! deps {
    () => {
        EscapeContainer!();
        Unescape!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl EscapeContainer for String { fn new () -> Self { Self :: new () } fn is_empty (& self) -> bool { self . is_empty () } fn push_str (& mut self , s : & str) { self . push_str (s) ; } fn push (& mut self , v : Unescape) { self . push (v . unwrap_char ()) ; } }
    };
}

impl_125!();