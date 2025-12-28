macro_rules! deps {
    () => {
        WriteStyle!();
        Writer!();
        Buffer!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl Writer { pub (crate) fn write_style (& self) -> WriteStyle { self . inner . write_style () } pub (crate) fn buffer (& self) -> Buffer { self . inner . buffer () } pub (crate) fn print (& self , buf : & Buffer) -> io :: Result < () > { self . inner . print (buf) } }
    };
}

impl_39!();