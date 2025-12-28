macro_rules! deps {
    () => {
        BufMut!();
        Shared!();
    };
}

macro_rules! BytesMut {
    () => {
        deps!();
        # [doc = " A unique reference to a contiguous slice of memory."] # [doc = ""] # [doc = " `BytesMut` represents a unique view into a potentially shared memory region."] # [doc = " Given the uniqueness guarantee, owners of `BytesMut` handles are able to"] # [doc = " mutate the memory."] # [doc = ""] # [doc = " `BytesMut` can be thought of as containing a `buf: Arc<Vec<u8>>`, an offset"] # [doc = " into `buf`, a slice length, and a guarantee that no other `BytesMut` for the"] # [doc = " same `buf` overlaps with its slice. That guarantee means that a write lock"] # [doc = " is not required."] # [doc = ""] # [doc = " # Growth"] # [doc = ""] # [doc = " `BytesMut`'s `BufMut` implementation will implicitly grow its buffer as"] # [doc = " necessary. However, explicitly reserving the required space up-front before"] # [doc = " a series of inserts will be more efficient."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bytes::{BytesMut, BufMut};"] # [doc = ""] # [doc = " let mut buf = BytesMut::with_capacity(64);"] # [doc = ""] # [doc = " buf.put_u8(b'h');"] # [doc = " buf.put_u8(b'e');"] # [doc = " buf.put(&b\"llo\"[..]);"] # [doc = ""] # [doc = " assert_eq!(&buf[..], b\"hello\");"] # [doc = ""] # [doc = " // Freeze the buffer so that it can be shared"] # [doc = " let a = buf.freeze();"] # [doc = ""] # [doc = " // This does not allocate, instead `b` points to the same memory."] # [doc = " let b = a.clone();"] # [doc = ""] # [doc = " assert_eq!(&a[..], b\"hello\");"] # [doc = " assert_eq!(&b[..], b\"hello\");"] # [doc = " ```"] pub struct BytesMut { ptr : NonNull < u8 > , len : usize , cap : usize , data : * mut Shared , }
    };
}

BytesMut!()