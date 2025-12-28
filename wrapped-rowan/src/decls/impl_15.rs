macro_rules! deps {
    () => {
        SyntaxNode!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Drop for SyntaxNode { # [inline] fn drop (& mut self) { if self . data () . dec_rc () { unsafe { free (self . ptr) } } } }
    };
}

impl_15!();