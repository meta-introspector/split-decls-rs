macro_rules! deps {
    () => {
        SyntaxToken!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Drop for SyntaxToken { # [inline] fn drop (& mut self) { if self . data () . dec_rc () { unsafe { free (self . ptr) } } } }
    };
}

impl_18!();