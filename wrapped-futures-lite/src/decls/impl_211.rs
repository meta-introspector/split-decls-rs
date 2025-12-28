macro_rules! deps {
    () => {
        BlockOn!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl < T : AsyncBufRead + Unpin > std :: io :: BufRead for BlockOn < T > { fn fill_buf (& mut self) -> Result < & [u8] > { future :: block_on (self . 0 . fill_buf ()) } fn consume (& mut self , amt : usize) { Pin :: new (& mut self . 0) . consume (amt) } }
    };
}

impl_211!();