macro_rules! deps {
    () => {
        ReadToEnd!();
    };
}

macro_rules! impl_1172 {
    () => {
        deps!();
        impl < A > Future for ReadToEnd < '_ , A > where A : AsyncRead + ? Sized + Unpin , { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; read_to_end_internal (Pin :: new (& mut this . reader) , cx , this . buf , this . start_len) } }
    };
}

impl_1172!()