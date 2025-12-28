macro_rules! deps {
    () => {
        Error!();
        HandleProgress!();
        ReadlineBufRead!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < 'a , T : io :: Read > ReadlineBufRead for WithSidebands < 'a , T , HandleProgress < 'a > > { fn readline (& mut self) -> Option < io :: Result < Result < PacketLineRef < '_ > , gix_packetline :: decode :: Error > > > { self . read_data_line () } fn readline_str (& mut self , line : & mut String) -> io :: Result < usize > { self . read_line_to_string (line) } }
    };
}

impl_118!()