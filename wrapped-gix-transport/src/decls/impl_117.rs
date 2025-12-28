macro_rules! deps {
    () => {
        Error!();
        ReadlineBufRead!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < T : io :: Read > ReadlineBufRead for WithSidebands < '_ , T , fn (bool , & [u8]) -> ProgressAction > { fn readline (& mut self) -> Option < io :: Result < Result < PacketLineRef < '_ > , gix_packetline :: decode :: Error > > > { self . read_data_line () } fn readline_str (& mut self , line : & mut String) -> io :: Result < usize > { self . read_line_to_string (line) } }
    };
}

impl_117!();