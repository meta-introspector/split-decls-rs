macro_rules! deps {
    () => {
        ChaCha20!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl Drop for ChaCha20 { fn drop (& mut self) { self . state . iter_mut () . zeroize () ; } }
    };
}

impl_339!()