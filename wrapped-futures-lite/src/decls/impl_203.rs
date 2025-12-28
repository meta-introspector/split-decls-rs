macro_rules! deps {
    () => {
        AsyncAsSync!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < T : AsyncSeek + Unpin > std :: io :: Seek for AsyncAsSync < '_ , '_ , T > { # [inline] fn seek (& mut self , pos : SeekFrom) -> Result < u64 > { self . poll_with (| io , cx | io . poll_seek (cx , pos)) } }
    };
}

impl_203!();