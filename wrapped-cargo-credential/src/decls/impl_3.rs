macro_rules! deps {
    () => {
        StringTypedError!();
        Error!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl From < anyhow :: Error > for Error { fn from (value : anyhow :: Error) -> Self { let mut prev = None ; for e in value . chain () . rev () { prev = Some (Box :: new (StringTypedError { message : e . to_string () , source : prev , })) ; } Error :: Other (prev . unwrap ()) } }
    };
}

impl_3!()