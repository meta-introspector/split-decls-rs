macro_rules! deps {
    () => {
        EscapeContainer!();
        Unescape!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl EscapeContainer for Vec < u8 > { fn new () -> Self { Self :: new () } fn is_empty (& self) -> bool { self . is_empty () } fn push_str (& mut self , s : & str) { self . extend_from_slice (s . as_bytes ()) ; } fn push (& mut self , v : Unescape) { match v { Unescape :: Byte (b) => self . push (b) , Unescape :: Unicode (c) => { let start = self . len () ; self . resize (self . len () + c . len_utf8 () , 0) ; c . encode_utf8 (& mut self [start ..]) ; } } } }
    };
}

impl_124!()