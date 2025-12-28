macro_rules! deps {
    () => {
        Result!();
        IntoUrlSealed!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl IntoUrlSealed for Url { fn into_url (self) -> crate :: Result < Url > { # [cfg (target_arch = "wasm32")] if self . scheme () == "blob" && self . path () . starts_with ("http") && self . as_str () [5 ..] . into_url () . is_ok () { return Ok (self) ; } if self . has_host () { Ok (self) } else { Err (crate :: error :: url_bad_scheme (self)) } } fn as_str (& self) -> & str { self . as_ref () } }
    };
}

impl_42!()