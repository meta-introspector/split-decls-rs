macro_rules! impl_226 {
    () => {
        impl < W : AsyncWrite + AsyncSeek > AsyncSeek for BufWriter < W > { # [doc = " Seek to the offset, in bytes, in the underlying writer."] # [doc = ""] # [doc = " Seeking always writes out the internal buffer before seeking."] fn poll_seek (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < Result < u64 > > { ready ! (self . as_mut () . poll_flush_buf (cx)) ? ; self . get_pin_mut () . poll_seek (cx , pos) } }
    };
}

impl_226!()