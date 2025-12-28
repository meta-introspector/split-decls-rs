macro_rules! deps {
    () => {
        V3!();
        UncompressedPublicKey!();
        AsymmetricPublicKey!();
        Error!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl TryFrom < & UncompressedPublicKey > for AsymmetricPublicKey < V3 > { type Error = Error ; fn try_from (value : & UncompressedPublicKey) -> Result < Self , Self :: Error > { Ok (Self { bytes : value . 0 . to_encoded_point (true) . as_ref () . to_vec () , phantom : PhantomData , }) } }
    };
}

impl_99!()