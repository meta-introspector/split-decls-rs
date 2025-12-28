macro_rules! deps {
    () => {
        ReadLineFuture!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl < R : AsyncBufRead + Unpin + ? Sized > Future for ReadLineFuture < '_ , R > { type Output = Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , buf , bytes , read , } = & mut * self ; read_line_internal (Pin :: new (reader) , cx , buf , bytes , read) } }
    };
}

impl_257!()