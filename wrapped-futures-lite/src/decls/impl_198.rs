macro_rules! deps {
    () => {
        AssertAsync!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl < T : std :: io :: Seek > AsyncSeek for AssertAsync < T > { # [inline] fn poll_seek (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < Result < u64 > > { assert_async_wrapio (move | | self . 0 . seek (pos)) } }
    };
}

impl_198!();