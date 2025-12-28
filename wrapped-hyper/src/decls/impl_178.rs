macro_rules! deps {
    () => {
        Result!();
        Write!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < P > Write for Pin < P > where P : DerefMut , P :: Target : Write , { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < std :: io :: Result < usize > > { pin_as_deref_mut (self) . poll_write (cx , buf) } fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [std :: io :: IoSlice < '_ >] ,) -> Poll < std :: io :: Result < usize > > { pin_as_deref_mut (self) . poll_write_vectored (cx , bufs) } fn is_write_vectored (& self) -> bool { (* * self) . is_write_vectored () } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < std :: io :: Result < () > > { pin_as_deref_mut (self) . poll_flush (cx) } fn poll_shutdown (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < std :: io :: Result < () > > { pin_as_deref_mut (self) . poll_shutdown (cx) } }
    };
}

impl_178!();