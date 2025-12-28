macro_rules! deps {
    () => {
        ReadHalf!();
    };
}

macro_rules! impl_1209 {
    () => {
        deps!();
        impl < R : AsyncRead > AsyncRead for ReadHalf < R > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { lock_and_then (& self . handle , cx , | l , cx | l . poll_read (cx , buf)) } fn poll_read_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < io :: Result < usize > > { lock_and_then (& self . handle , cx , | l , cx | l . poll_read_vectored (cx , bufs)) } }
    };
}

impl_1209!()