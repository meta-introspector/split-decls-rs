macro_rules! _unsized_std_propagation {
    () => {
        # [cfg (feature = "std")] fn _unsized_std_propagation () { check_t ! (:: std :: path :: Path) ; check_t ! (:: std :: ffi :: OsStr) ; check_t ! (:: std :: ffi :: CStr) ; }
    };
}

_unsized_std_propagation!()