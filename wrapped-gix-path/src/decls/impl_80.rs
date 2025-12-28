macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < 'a > TryFrom < & 'a BString > for & 'a RelativePath { type Error = Error ; fn try_from (value : & 'a BString) -> Result < Self , Self :: Error > { let path = try_from_bstr (value . as_bstr ()) ? ; relative_path_from_value_and_path (value . as_bstr () , & path) } }
    };
}

impl_80!();