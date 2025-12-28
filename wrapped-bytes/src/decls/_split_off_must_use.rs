macro_rules! _split_off_must_use {
    () => {
        # [doc = " ```compile_fail"] # [doc = " use bytes::BytesMut;"] # [doc = " #[deny(unused_must_use)]"] # [doc = " {"] # [doc = "     let mut b1 = BytesMut::from(\"hello world\");"] # [doc = "     b1.split_off(6);"] # [doc = " }"] # [doc = " ```"] fn _split_off_must_use () { }
    };
}

_split_off_must_use!()