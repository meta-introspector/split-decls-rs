macro_rules! deps {
    () => {
        DecodeError!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        # [allow (clippy :: from_over_into)] impl Into < crate :: error :: DecodeError > for DecodeError { fn into (self) -> crate :: error :: DecodeError { crate :: error :: DecodeError :: Serde (self) } }
    };
}

impl_179!();