macro_rules! deps {
    () => {
        Context!();
        NextAction!();
        Error!();
        Result!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl TryFrom < & NextAction > for Context { type Error = protocol :: context :: decode :: Error ; fn try_from (value : & NextAction) -> std :: result :: Result < Self , Self :: Error > { Context :: from_bytes (value . previous_output . as_ref ()) } }
    };
}

impl_10!();