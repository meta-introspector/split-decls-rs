macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < 'a , const N : usize > TryFrom < & 'a [u8 ; N] > for & 'a RelativePath { type Error = Error ; # [inline] fn try_from (value : & 'a [u8 ; N]) -> Result < Self , Self :: Error > { let path = try_from_byte_slice (value . as_bstr ()) ? ; relative_path_from_value_and_path (value . as_bstr () , path) } }
    };
}

impl_79!();