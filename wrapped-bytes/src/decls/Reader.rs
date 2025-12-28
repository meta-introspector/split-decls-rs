macro_rules! deps {
    () => {
        Buf!();
    };
}

macro_rules! Reader {
    () => {
        deps!();
        # [doc = " A `Buf` adapter which implements `io::Read` for the inner value."] # [doc = ""] # [doc = " This struct is generally created by calling `reader()` on `Buf`. See"] # [doc = " documentation of [`reader()`](Buf::reader) for more"] # [doc = " details."] # [derive (Debug)] pub struct Reader < B > { buf : B , }
    };
}

Reader!();