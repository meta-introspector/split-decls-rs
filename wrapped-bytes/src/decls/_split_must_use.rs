macro_rules! _split_must_use {
    () => {
        # [doc = " ```compile_fail"] # [doc = " use bytes::BytesMut;"] # [doc = " #[deny(unused_must_use)]"] # [doc = " {"] # [doc = "     let mut b1 = BytesMut::from(\"hello world\");"] # [doc = "     b1.split();"] # [doc = " }"] # [doc = " ```"] fn _split_must_use () { }
    };
}

_split_must_use!();