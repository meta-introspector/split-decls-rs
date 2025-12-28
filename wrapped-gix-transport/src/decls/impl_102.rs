macro_rules! deps {
    () => {
        ReadlineBufRead!();
        HeadersThenBody!();
        Error!();
        Http!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < H : Http , B : ReadlineBufRead + Unpin > ReadlineBufRead for HeadersThenBody < H , B > { fn readline (& mut self) -> Option < std :: io :: Result < Result < PacketLineRef < '_ > , gix_packetline :: decode :: Error > > > { if let Err (err) = self . handle_headers () { return Some (Err (err)) ; } self . body . readline () } fn readline_str (& mut self , line : & mut String) -> std :: io :: Result < usize > { self . handle_headers () ? ; self . body . readline_str (line) } }
    };
}

impl_102!()