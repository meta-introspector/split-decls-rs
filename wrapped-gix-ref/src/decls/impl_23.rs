macro_rules! deps {
    () => {
        Category!();
        PartialNameRef!();
        FullNameRef!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl PartialNameRef { pub (crate) fn looks_like_full_name (& self , consider_pseudo_ref : bool) -> bool { let name = self . 0 . as_bstr () ; name . starts_with_str ("refs/") || name . starts_with (Category :: MainPseudoRef . prefix ()) || name . starts_with (Category :: LinkedPseudoRef { name : "" . into () } . prefix ()) || (consider_pseudo_ref && is_pseudo_ref (name)) } pub (crate) fn construct_full_name_ref < 'buf > (& self , inbetween : & str , buf : & 'buf mut BString , consider_pseudo_ref : bool ,) -> & 'buf FullNameRef { buf . clear () ; if ! self . looks_like_full_name (consider_pseudo_ref) { buf . push_str ("refs/") ; } if ! inbetween . is_empty () { buf . push_str (inbetween) ; buf . push_byte (b'/') ; } buf . extend_from_slice (& self . 0) ; FullNameRef :: new_unchecked (buf . as_bstr ()) } }
    };
}

impl_23!();