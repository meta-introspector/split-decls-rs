macro_rules! deps {
    () => {
        AssertAsync!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl < T : std :: io :: Read > AsyncRead for AssertAsync < T > { # [inline] fn poll_read (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize > > { assert_async_wrapio (move | | self . 0 . read (buf)) } # [inline] fn poll_read_vectored (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < Result < usize > > { assert_async_wrapio (move | | self . 0 . read_vectored (bufs)) } }
    };
}

impl_196!();