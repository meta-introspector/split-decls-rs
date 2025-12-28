macro_rules! deps {
    () => {
        Namespace!();
        FullName!();
        Category!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl FullName { # [doc = " Convert this name into the relative path, lossily, identifying the reference location relative to a repository"] pub fn to_path (& self) -> & Path { gix_path :: from_byte_slice (& self . 0) } # [doc = " Dissolve this instance and return the buffer."] pub fn into_inner (self) -> BString { self . 0 } # [doc = " Return ourselves as byte string which is a valid refname"] pub fn as_bstr (& self) -> & BStr { self . 0 . as_bstr () } # [doc = " Modify ourself so that we use `namespace` as prefix, if it is not yet in the `namespace`"] pub fn prefix_namespace (& mut self , namespace : & Namespace) -> & mut Self { if ! self . 0 . starts_with_str (& namespace . 0) { self . 0 . insert_str (0 , & namespace . 0) ; } self } # [doc = " Strip the given `namespace` off the beginning of this name, if it is in this namespace."] pub fn strip_namespace (& mut self , namespace : & Namespace) -> & mut Self { if self . 0 . starts_with_str (& namespace . 0) { let prev_len = self . 0 . len () ; self . 0 . copy_within (namespace . 0 . len () .. , 0) ; self . 0 . resize (prev_len - namespace . 0 . len () , 0) ; } self } # [doc = " Strip well-known prefixes from the name and return it."] # [doc = ""] # [doc = " If there is no such prefix, the original name is returned."] pub fn shorten (& self) -> & BStr { self . as_ref () . shorten () } # [doc = " Classify this name, or return `None` if it's unclassified."] pub fn category (& self) -> Option < crate :: Category < '_ > > { self . as_ref () . category () } # [doc = " Classify this name, or return `None` if it's unclassified. If `Some`,"] # [doc = " the shortened name is returned as well."] pub fn category_and_short_name (& self) -> Option < (crate :: Category < '_ > , & BStr) > { self . as_ref () . category_and_short_name () } }
    };
}

impl_13!()