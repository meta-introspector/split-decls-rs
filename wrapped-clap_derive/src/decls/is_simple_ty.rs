macro_rules! is_simple_ty {
    () => {
        pub (crate) fn is_simple_ty (ty : & Type , name : & str) -> bool { only_last_segment (ty) . map (| segment | { if let PathArguments :: None = segment . arguments { segment . ident == name } else { false } }) . unwrap_or (false) }
    };
}

is_simple_ty!()