macro_rules! deps {
    () => {
        ReadToString!();
    };
}

macro_rules! impl_1178 {
    () => {
        deps!();
        impl < A > Future for ReadToString < '_ , A > where A : AsyncRead + ? Sized + Unpin , { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , buf , bytes , start_len } = & mut * self ; read_to_string_internal (Pin :: new (reader) , cx , buf , bytes , * start_len) } }
    };
}

impl_1178!();