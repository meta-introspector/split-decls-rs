macro_rules! deps {
    () => {
        Transaction!();
    };
}

macro_rules! impl_806 {
    () => {
        deps!();
        impl Drop for Transaction < '_ > { fn drop (& mut self) { unsafe { raw :: git_transaction_free (self . raw) } } }
    };
}

impl_806!();