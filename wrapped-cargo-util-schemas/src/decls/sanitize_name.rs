macro_rules! sanitize_name {
    () => {
        pub (crate) fn sanitize_name (name : & str , placeholder : char) -> String { let mut slug = String :: new () ; let mut chars = name . chars () ; while let Some (ch) = chars . next () { if (unicode_ident :: is_xid_start (ch) || ch == '_') && ! ch . is_digit (10) { slug . push (ch) ; break ; } } while let Some (ch) = chars . next () { if unicode_ident :: is_xid_continue (ch) || ch == '-' { slug . push (ch) ; } else { slug . push (placeholder) ; } } if slug . is_empty () { slug . push_str ("package") ; } slug }
    };
}

sanitize_name!();