macro_rules! deps {
    () => {
        ReadlineBufRead!();
        Error!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        # [async_trait (? Send)] impl < T : ReadlineBufRead + ? Sized + Unpin > ReadlineBufRead for Box < T > { async fn readline (& mut self) -> Option < io :: Result < Result < PacketLineRef < '_ > , gix_packetline :: decode :: Error > > > { self . deref_mut () . readline () . await } async fn readline_str (& mut self , line : & mut String) -> io :: Result < usize > { self . deref_mut () . readline_str (line) . await } }
    };
}

impl_7!()