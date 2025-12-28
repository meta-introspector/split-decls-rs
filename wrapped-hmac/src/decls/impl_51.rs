macro_rules! impl_51 {
    () => {
        impl < D : EagerHash + AlgorithmName > AlgorithmName for Hmac < D > { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < Self as CoreProxy > :: Core :: write_alg_name (f) } }
    };
}

impl_51!()