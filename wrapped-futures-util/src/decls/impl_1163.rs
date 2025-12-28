macro_rules! deps {
    () => {
        Ready!();
        ReadLine!();
    };
}

macro_rules! impl_1163 {
    () => {
        deps!();
        impl < R : AsyncBufRead + ? Sized + Unpin > Future for ReadLine < '_ , R > { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , buf , bytes , read , finished : _ } = & mut * self ; let ret = ready ! (read_line_internal (Pin :: new (reader) , cx , buf , bytes , read)) ; self . finished = true ; Poll :: Ready (ret) } }
    };
}

impl_1163!()