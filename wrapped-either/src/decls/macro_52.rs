macro_rules! macro_52 {
    () => {
        impl_specific_ref_and_mut ! (:: std :: ffi :: CStr , cfg (feature = "std") , doc = "Requires crate feature `std`.") ;
    };
}

macro_52!()