macro_rules! deps {
    () => {
        AssertAsync!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < T : std :: io :: Write > AsyncWrite for AssertAsync < T > { # [inline] fn poll_write (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize > > { assert_async_wrapio (move | | self . 0 . write (buf)) } # [inline] fn poll_write_vectored (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < Result < usize > > { assert_async_wrapio (move | | self . 0 . write_vectored (bufs)) } # [inline] fn poll_flush (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () > > { assert_async_wrapio (move | | self . 0 . flush ()) } # [inline] fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { self . poll_flush (cx) } }
    };
}

impl_197!()