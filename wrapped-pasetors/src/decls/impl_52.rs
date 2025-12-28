macro_rules! deps {
    () => {
        V4!();
        SymmetricKey!();
        Error!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        # [cfg (feature = "v4")] impl TryFrom < & str > for SymmetricKey < V4 > { type Error = Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Ok (Self { bytes : validate_paserk_string (value , "k4" , "local" , V4 :: LOCAL_KEY) ? , phantom : PhantomData , }) } }
    };
}

impl_52!();