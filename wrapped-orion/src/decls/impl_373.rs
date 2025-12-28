macro_rules! deps {
    () => {
        Scalar!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        impl Drop for Scalar { fn drop (& mut self) { use zeroize :: Zeroize ; self . 0 . iter_mut () . zeroize () ; } }
    };
}

impl_373!();