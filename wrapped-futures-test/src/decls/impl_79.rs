macro_rules! deps {
    () => {
        AssertUnmoved!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < S : AsyncSeek > AsyncSeek for AssertUnmoved < S > { fn poll_seek (self : Pin < & mut Self > , cx : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < io :: Result < u64 > > { self . poll_with (| s | s . poll_seek (cx , pos)) } }
    };
}

impl_79!()