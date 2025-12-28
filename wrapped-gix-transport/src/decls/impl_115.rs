macro_rules! deps {
    () => {
        ReadlineBufRead!();
        Error!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < T : ReadlineBufRead + ? Sized > ReadlineBufRead for Box < T > { fn readline (& mut self) -> Option < io :: Result < Result < PacketLineRef < '_ > , gix_packetline :: decode :: Error > > > { ReadlineBufRead :: readline (self . deref_mut ()) } fn readline_str (& mut self , line : & mut String) -> io :: Result < usize > { ReadlineBufRead :: readline_str (self . deref_mut () , line) } }
    };
}

impl_115!();