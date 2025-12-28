macro_rules! deps {
    () => {
        BlockOn!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl < T : AsyncRead + Unpin > std :: io :: Read for BlockOn < T > { fn read (& mut self , buf : & mut [u8]) -> Result < usize > { future :: block_on (self . 0 . read (buf)) } }
    };
}

impl_210!()