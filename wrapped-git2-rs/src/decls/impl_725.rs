macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! impl_725 {
    () => {
        deps!();
        impl < 'a > Drop for Signature < 'a > { fn drop (& mut self) { if self . owned { unsafe { raw :: git_signature_free (self . raw) } } } }
    };
}

impl_725!()