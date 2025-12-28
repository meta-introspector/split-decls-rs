macro_rules! deps {
    () => {
        Buf!();
        Bytes!();
        BufMut!();
    };
}

macro_rules! Chain {
    () => {
        deps!();
        # [doc = " A `Chain` sequences two buffers."] # [doc = ""] # [doc = " `Chain` is an adapter that links two underlying buffers and provides a"] # [doc = " continuous view across both buffers. It is able to sequence either immutable"] # [doc = " buffers ([`Buf`] values) or mutable buffers ([`BufMut`] values)."] # [doc = ""] # [doc = " This struct is generally created by calling [`Buf::chain`]. Please see that"] # [doc = " function's documentation for more detail."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bytes::{Bytes, Buf};"] # [doc = ""] # [doc = " let mut buf = (&b\"hello \"[..])"] # [doc = "     .chain(&b\"world\"[..]);"] # [doc = ""] # [doc = " let full: Bytes = buf.copy_to_bytes(11);"] # [doc = " assert_eq!(full[..], b\"hello world\"[..]);"] # [doc = " ```"] # [doc = ""] # [doc = " [`Buf::chain`]: Buf::chain"] # [derive (Debug)] pub struct Chain < T , U > { a : T , b : U , }
    };
}

Chain!()