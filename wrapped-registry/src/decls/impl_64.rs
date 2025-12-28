macro_rules! deps {
    () => {
        Transaction!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl Drop for Transaction { fn drop (& mut self) { unsafe { CloseHandle (self . 0) } ; } }
    };
}

impl_64!();