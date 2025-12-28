macro_rules! deps {
    () => {
        Poly1305!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl Drop for Poly1305 { fn drop (& mut self) { use zeroize :: Zeroize ; self . a . 0 . zeroize () ; self . r . 0 . zeroize () ; self . s . zeroize () ; self . buffer . zeroize () ; } }
    };
}

impl_262!()