macro_rules! other_264 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Compares ``lhs->name`` of length ``lhs->namelen`` bytes and"] # [doc = " ``rhs->name`` of length ``rhs->namelen`` bytes.  Returns negative"] # [doc = " integer if ``lhs->name`` is found to be less than ``rhs->name``; or"] # [doc = " returns positive integer if ``lhs->name`` is found to be greater"] # [doc = " than ``rhs->name``; or returns 0 otherwise."] pub fn nghttp2_nv_compare_name (lhs : * const nghttp2_nv , rhs : * const nghttp2_nv ,) -> :: std :: os :: raw :: c_int ; }
    };
}

other_264!();