macro_rules! deps {
    () => {
        AsyncAsSync!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl < T : AsyncWrite + Unpin > std :: io :: Write for AsyncAsSync < '_ , '_ , T > { # [inline] fn write (& mut self , buf : & [u8]) -> Result < usize > { self . poll_with (| io , cx | io . poll_write (cx , buf)) } # [inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> Result < usize > { self . poll_with (| io , cx | io . poll_write_vectored (cx , bufs)) } # [inline] fn flush (& mut self) -> Result < () > { self . poll_with (| io , cx | io . poll_flush (cx)) } }
    };
}

impl_202!();