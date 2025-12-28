macro_rules! deps {
    () => {
        Binding!();
        BlameOptions!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl Binding for BlameOptions { type Raw = * mut raw :: git_blame_options ; unsafe fn from_raw (opts : * mut raw :: git_blame_options) -> BlameOptions { BlameOptions { raw : * opts } } fn raw (& self) -> * mut raw :: git_blame_options { & self . raw as * const _ as * mut _ } }
    };
}

impl_208!()