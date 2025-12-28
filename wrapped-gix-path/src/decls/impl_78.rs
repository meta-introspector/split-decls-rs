macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < 'a > TryFrom < & 'a [u8] > for & 'a RelativePath { type Error = Error ; # [inline] fn try_from (value : & 'a [u8]) -> Result < Self , Self :: Error > { let path = try_from_byte_slice (value) ? ; relative_path_from_value_and_path (value . as_bstr () , path) } }
    };
}

impl_78!()