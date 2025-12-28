macro_rules! deps {
    () => {
        Path!();
        Boolean!();
        Error!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl TryFrom < OsString > for Boolean { type Error = Error ; fn try_from (value : OsString) -> Result < Self , Self :: Error > { let value = gix_path :: os_str_into_bstr (& value) . map_err (| _ | Error :: new ("Illformed UTF-8" , std :: path :: Path :: new (& value) . display () . to_string ())) ? ; Self :: try_from (value) } }
    };
}

impl_3!()