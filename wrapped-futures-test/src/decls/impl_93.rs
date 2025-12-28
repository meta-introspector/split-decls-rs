macro_rules! deps {
    () => {
        InterleavePending!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < S : AsyncSeek > AsyncSeek for InterleavePending < S > { fn poll_seek (self : Pin < & mut Self > , cx : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < io :: Result < u64 > > { self . poll_with (cx , | s , cx | s . poll_seek (cx , pos)) } }
    };
}

impl_93!();