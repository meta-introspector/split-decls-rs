macro_rules! eq_case {
    () => {
        # [doc = " A const implementation of case-insensitive ASCII equals."] const fn eq_case (lhs : & [u8] , rhs : & [u8]) -> bool { if lhs . len () != rhs . len () { return false ; } let mut i = 0usize ; while i < lhs . len () { if ! lhs [i] . eq_ignore_ascii_case (& rhs [i]) { return false ; } i += 1 ; } true }
    };
}

eq_case!()