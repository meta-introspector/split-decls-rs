macro_rules! deps {
    () => {
        Bytes!();
        BytesMut!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl From < Bytes > for BytesMut { # [doc = " Convert self into `BytesMut`."] # [doc = ""] # [doc = " If `bytes` is unique for the entire original buffer, this will return a"] # [doc = " `BytesMut` with the contents of `bytes` without copying."] # [doc = " If `bytes` is not unique for the entire original buffer, this will make"] # [doc = " a copy of `bytes` subset of the original buffer in a new `BytesMut`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bytes::{Bytes, BytesMut};"] # [doc = ""] # [doc = " let bytes = Bytes::from(b\"hello\".to_vec());"] # [doc = " assert_eq!(BytesMut::from(bytes), BytesMut::from(&b\"hello\"[..]));"] # [doc = " ```"] fn from (bytes : Bytes) -> Self { let bytes = ManuallyDrop :: new (bytes) ; unsafe { (bytes . vtable . into_mut) (& bytes . data , bytes . ptr , bytes . len) } } }
    };
}

impl_111!()