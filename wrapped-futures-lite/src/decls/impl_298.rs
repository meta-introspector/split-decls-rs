macro_rules! deps {
    () => {
        SeekFuture!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < S : AsyncSeek + Unpin + ? Sized > Future for SeekFuture < '_ , S > { type Output = Result < u64 > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let pos = self . pos ; Pin :: new (& mut * self . seeker) . poll_seek (cx , pos) } }
    };
}

impl_298!()