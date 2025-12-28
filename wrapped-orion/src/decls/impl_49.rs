macro_rules! deps {
    () => {
        U32x4!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl zeroize :: Zeroize for U32x4 { fn zeroize (& mut self) { self . 0 . zeroize () ; self . 1 . zeroize () ; self . 2 . zeroize () ; self . 3 . zeroize () ; } }
    };
}

impl_49!()