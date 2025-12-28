macro_rules! deps {
    () => {
        DescribeOptions!();
        Binding!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl Binding for DescribeOptions { type Raw = * mut raw :: git_describe_options ; unsafe fn from_raw (_raw : * mut raw :: git_describe_options) -> DescribeOptions { panic ! ("unimplemened") } fn raw (& self) -> * mut raw :: git_describe_options { & self . raw as * const _ as * mut _ } }
    };
}

impl_297!()