macro_rules! _split_to_must_use {
    () => {
        # [doc = " ```compile_fail"] # [doc = " use bytes::BytesMut;"] # [doc = " #[deny(unused_must_use)]"] # [doc = " {"] # [doc = "     let mut b1 = BytesMut::from(\"hello world\");"] # [doc = "     b1.split_to(6);"] # [doc = " }"] # [doc = " ```"] fn _split_to_must_use () { }
    };
}

_split_to_must_use!();