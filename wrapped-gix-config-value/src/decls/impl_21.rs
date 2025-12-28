macro_rules! deps {
    () => {
        Error!();
        Name!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl TryFrom < & BStr > for Name { type Error = Error ; fn try_from (s : & BStr) -> Result < Self , Self :: Error > { Self :: from_str (std :: str :: from_utf8 (s) . map_err (| err | color_err (s) . with_err (err)) ?) } }
    };
}

impl_21!();