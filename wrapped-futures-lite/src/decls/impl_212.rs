macro_rules! deps {
    () => {
        BlockOn!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < T : AsyncWrite + Unpin > std :: io :: Write for BlockOn < T > { fn write (& mut self , buf : & [u8]) -> Result < usize > { future :: block_on (self . 0 . write (buf)) } fn flush (& mut self) -> Result < () > { future :: block_on (self . 0 . flush ()) } }
    };
}

impl_212!()