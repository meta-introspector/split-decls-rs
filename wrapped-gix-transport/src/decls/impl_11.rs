macro_rules! deps {
    () => {
        ExtendedBufRead!();
        Protocol!();
        MessageKind!();
        Error!();
        HandleProgress!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        # [async_trait (? Send)] impl < 'a , T : AsyncRead + Unpin > ExtendedBufRead < 'a > for WithSidebands < 'a , T , HandleProgress < 'a > > { fn set_progress_handler (& mut self , handle_progress : Option < HandleProgress < 'a > >) { self . set_progress_handler (handle_progress) ; } async fn peek_data_line (& mut self) -> Option < io :: Result < Result < & [u8] , Error > > > { match self . peek_data_line () . await { Some (Ok (Ok (line))) => Some (Ok (Ok (line))) , Some (Ok (Err (err))) => Some (Ok (Err (err . into ()))) , Some (Err (err)) => Some (Err (err)) , None => None , } } fn reset (& mut self , version : Protocol) { match version { Protocol :: V0 | Protocol :: V1 => self . reset_with (& [gix_packetline :: PacketLineRef :: Flush]) , Protocol :: V2 => self . reset_with (& [gix_packetline :: PacketLineRef :: Delimiter , gix_packetline :: PacketLineRef :: Flush ,]) , } } fn stopped_at (& self) -> Option < MessageKind > { self . stopped_at () . map (| l | match l { gix_packetline :: PacketLineRef :: Flush => MessageKind :: Flush , gix_packetline :: PacketLineRef :: Delimiter => MessageKind :: Delimiter , gix_packetline :: PacketLineRef :: ResponseEnd => MessageKind :: ResponseEnd , gix_packetline :: PacketLineRef :: Data (_) => unreachable ! ("data cannot be a delimiter") , }) } }
    };
}

impl_11!();