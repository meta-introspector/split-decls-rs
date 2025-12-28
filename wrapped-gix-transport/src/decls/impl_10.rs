macro_rules! deps {
    () => {
        HandleProgress!();
        Error!();
        ReadlineBufRead!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        # [async_trait (? Send)] impl < 'a , T : AsyncRead + Unpin > ReadlineBufRead for WithSidebands < 'a , T , HandleProgress < 'a > > { async fn readline (& mut self) -> Option < io :: Result < Result < PacketLineRef < '_ > , gix_packetline :: decode :: Error > > > { self . read_data_line () . await } async fn readline_str (& mut self , line : & mut String) -> io :: Result < usize > { self . read_line_to_string (line) . await } }
    };
}

impl_10!()