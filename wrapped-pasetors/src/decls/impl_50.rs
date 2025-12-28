macro_rules! deps {
    () => {
        Error!();
        SymmetricKey!();
        V2!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        # [cfg (feature = "v2")] impl TryFrom < & str > for SymmetricKey < V2 > { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Ok (Self { bytes : validate_paserk_string (value , "k2" , "local" , V2 :: LOCAL_KEY) ? , phantom : PhantomData , }) } }
    };
}

impl_50!();