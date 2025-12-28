macro_rules! deps {
    () => {
        BlockOn!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl < T : AsyncSeek + Unpin > std :: io :: Seek for BlockOn < T > { fn seek (& mut self , pos : SeekFrom) -> Result < u64 > { future :: block_on (self . 0 . seek (pos)) } }
    };
}

impl_213!()