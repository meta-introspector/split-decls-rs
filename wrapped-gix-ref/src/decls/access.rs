macro_rules! deps {
    () => {
        Reference!();
        Namespace!();
        FullNameRef!();
        Target!();
        Kind!();
    };
}

macro_rules! access {
    () => {
        deps!();
        mod access { use gix_object :: bstr :: ByteSlice ; use crate :: { raw :: Reference , FullNameRef , Namespace , Target } ; impl Reference { # [doc = " Returns the kind of reference based on its target"] pub fn kind (& self) -> crate :: Kind { self . target . kind () } # [doc = " Return the full validated name of the reference, with the given namespace stripped if possible."] # [doc = ""] # [doc = " If the reference name wasn't prefixed with `namespace`, `None` is returned instead."] pub fn name_without_namespace (& self , namespace : & Namespace) -> Option < & FullNameRef > { self . name . 0 . as_bstr () . strip_prefix (namespace . 0 . as_bytes ()) . map (| stripped | FullNameRef :: new_unchecked (stripped . as_bstr ())) } # [doc = " Strip the given namespace from our name as well as the name, but not the reference we point to."] pub fn strip_namespace (& mut self , namespace : & Namespace) -> & mut Self { self . name . strip_namespace (namespace) ; if let Target :: Symbolic (name) = & mut self . target { name . strip_namespace (namespace) ; } self } } }
    };
}

access!();