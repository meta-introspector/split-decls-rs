macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < 'a > TryFrom < & 'a BStr > for & 'a RelativePath { type Error = Error ; fn try_from (value : & 'a BStr) -> Result < Self , Self :: Error > { let path = try_from_bstr (value) ? ; relative_path_from_value_and_path (value , & path) } }
    };
}

impl_77!()