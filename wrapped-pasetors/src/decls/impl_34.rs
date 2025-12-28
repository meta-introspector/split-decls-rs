macro_rules! deps {
    () => {
        SymmetricKey!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < V > Drop for SymmetricKey < V > { fn drop (& mut self) { use zeroize :: Zeroize ; self . bytes . iter_mut () . zeroize () ; } }
    };
}

impl_34!()