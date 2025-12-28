macro_rules! deps {
    () => {
        Error!();
        PartialNameRef!();
        FullName!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        # [allow (clippy :: infallible_try_from)] impl < 'a > convert :: TryFrom < & 'a FullName > for & 'a PartialNameRef { type Error = Infallible ; fn try_from (v : & 'a FullName) -> Result < Self , Self :: Error > { Ok (v . as_ref () . as_partial_name ()) } }
    };
}

impl_36!();