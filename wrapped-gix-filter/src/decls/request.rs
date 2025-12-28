macro_rules! deps {
    () => {
        Request!();
        Status!();
        PacketlineReader!();
    };
}

macro_rules! request {
    () => {
        deps!();
        mod request { use std :: io :: Write ; use gix_packetline :: blocking_io :: { encode , Writer } ; use crate :: driver :: { process , process :: { server :: Request , PacketlineReader } , } ; impl Request < '_ > { # [doc = " Turn ourselves into a reader that can read until the next flush packet."] pub fn as_read (& mut self) -> PacketlineReader < '_ , std :: io :: StdinLock < 'static > > { self . parent . input . as_read () } # [doc = " Provide the write-end of the underlying process."] pub fn as_write (& mut self) -> impl std :: io :: Write + '_ { WriteAndFlushOnDrop { inner : & mut self . parent . out , } } # [doc = " Write the `status` message followed by a flush packet."] pub fn write_status (& mut self , status : process :: Status) -> std :: io :: Result < () > { let out = & mut self . parent . out ; if let Some (message) = status . message () { out . write_all (format ! ("status={message}") . as_bytes ()) ? ; } encode :: flush_to_write (out . inner_mut ()) ? ; out . flush () } } impl std :: fmt :: Debug for Request < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Request") . field ("command" , & self . command) . field ("meta" , & self . meta) . finish () } } struct WriteAndFlushOnDrop < 'a > { inner : & 'a mut Writer < std :: io :: StdoutLock < 'static > > , } impl std :: io :: Write for WriteAndFlushOnDrop < '_ > { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . inner . write (buf) } fn flush (& mut self) -> std :: io :: Result < () > { self . inner . flush () } } impl Drop for WriteAndFlushOnDrop < '_ > { fn drop (& mut self) { encode :: flush_to_write (self . inner . inner_mut ()) . ok () ; self . inner . flush () . ok () ; } } }
    };
}

request!();