macro_rules! deps {
    () => {
        Read!();
    };
}

macro_rules! impl_1147 {
    () => {
        deps!();
        impl < R : AsyncRead + ? Sized + Unpin > Future for Read < '_ , R > { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; Pin :: new (& mut this . reader) . poll_read (cx , this . buf) } }
    };
}

impl_1147!()