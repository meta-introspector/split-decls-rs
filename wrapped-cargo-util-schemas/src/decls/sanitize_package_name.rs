macro_rules! sanitize_package_name {
    () => {
        # [doc = " Ensure a package name is [valid][validate_package_name]"] pub (crate) fn sanitize_package_name (name : & str , placeholder : char) -> String { let mut slug = String :: new () ; for part in name . split ("::") { if ! slug . is_empty () { slug . push_str ("::") ; } slug . push_str (& sanitize_name (part , placeholder)) ; } slug }
    };
}

sanitize_package_name!();