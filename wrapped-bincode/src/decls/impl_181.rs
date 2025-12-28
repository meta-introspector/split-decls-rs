macro_rules! deps {
    () => {
        EncodeError!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        # [allow (clippy :: from_over_into)] impl Into < crate :: error :: EncodeError > for EncodeError { fn into (self) -> crate :: error :: EncodeError { crate :: error :: EncodeError :: Serde (self) } }
    };
}

impl_181!()