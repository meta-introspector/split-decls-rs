macro_rules! deps {
    () => {
        Read!();
        ReadBufCursor!();
        Result!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < P > Read for Pin < P > where P : DerefMut , P :: Target : Read , { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : ReadBufCursor < '_ > ,) -> Poll < std :: io :: Result < () > > { pin_as_deref_mut (self) . poll_read (cx , buf) } }
    };
}

impl_174!()