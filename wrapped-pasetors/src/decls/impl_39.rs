macro_rules! deps {
    () => {
        AsymmetricSecretKey!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < V > Drop for AsymmetricSecretKey < V > { fn drop (& mut self) { use zeroize :: Zeroize ; self . bytes . iter_mut () . zeroize () ; } }
    };
}

impl_39!();