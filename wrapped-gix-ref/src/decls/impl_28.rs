macro_rules! deps {
    () => {
        PartialNameRef!();
        Error!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < 'a > convert :: TryFrom < & 'a OsStr > for & 'a PartialNameRef { type Error = Error ; fn try_from (v : & 'a OsStr) -> Result < Self , Self :: Error > { let v = gix_path :: os_str_into_bstr (v) . map_err (| _ | { Error :: Tag (gix_validate :: tag :: name :: Error :: InvalidByte { byte : "<unknown encoding>" . into () , }) }) ? ; Ok (PartialNameRef :: new_unchecked (gix_validate :: reference :: name_partial (v . as_bstr () ,) ?)) } }
    };
}

impl_28!()