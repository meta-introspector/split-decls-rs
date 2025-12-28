macro_rules! deps {
    () => {
        BufMut!();
    };
}

macro_rules! Writer {
    () => {
        deps!();
        # [doc = " A `BufMut` adapter which implements `io::Write` for the inner value."] # [doc = ""] # [doc = " This struct is generally created by calling `writer()` on `BufMut`. See"] # [doc = " documentation of [`writer()`](BufMut::writer) for more"] # [doc = " details."] # [derive (Debug)] pub struct Writer < B > { buf : B , }
    };
}

Writer!()