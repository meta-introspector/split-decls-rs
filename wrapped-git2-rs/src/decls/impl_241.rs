macro_rules! deps {
    () => {
        Binding!();
        Buf!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl Binding for Buf { type Raw = * mut raw :: git_buf ; unsafe fn from_raw (raw : * mut raw :: git_buf) -> Buf { Buf { raw : * raw } } fn raw (& self) -> * mut raw :: git_buf { & self . raw as * const _ as * mut _ } }
    };
}

impl_241!();