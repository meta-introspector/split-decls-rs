macro_rules! deps {
    () => {
        ExternAbi!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl ExternAbi { # [doc = " Default ABI chosen for `extern fn` declarations without an explicit ABI."] pub const FALLBACK : ExternAbi = ExternAbi :: C { unwind : false } ; pub fn name (self) -> & 'static str { self . as_str () } }
    };
}

impl_29!()