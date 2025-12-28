macro_rules! deps {
    () => {
        Value!();
        ValueRef!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        # [doc = " Access"] impl Value { # [doc = " Return ourselves as reference."] pub fn as_ref (& self) -> ValueRef < '_ > { ValueRef (self . 0 . as_ref ()) } }
    };
}

impl_19!();