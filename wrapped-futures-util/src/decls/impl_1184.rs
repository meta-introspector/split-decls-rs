macro_rules! deps {
    () => {
        ReadUntil!();
    };
}

macro_rules! impl_1184 {
    () => {
        deps!();
        impl < R : AsyncBufRead + ? Sized + Unpin > Future for ReadUntil < '_ , R > { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , byte , buf , read } = & mut * self ; read_until_internal (Pin :: new (reader) , cx , * byte , buf , read) } }
    };
}

impl_1184!();