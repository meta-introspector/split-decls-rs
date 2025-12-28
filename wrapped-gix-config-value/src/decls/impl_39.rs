macro_rules! deps {
    () => {
        Error!();
        Suffix!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl TryFrom < & BStr > for Suffix { type Error = () ; fn try_from (s : & BStr) -> Result < Self , Self :: Error > { Self :: from_str (std :: str :: from_utf8 (s) . map_err (| _ | ()) ?) } }
    };
}

impl_39!()