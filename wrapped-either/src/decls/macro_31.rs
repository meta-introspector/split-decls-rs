macro_rules! macro_31 {
    () => {
        impl_specific_ref_and_mut ! (:: std :: ffi :: OsStr , cfg (feature = "std") , doc = "Requires crate feature `std`.") ;
    };
}

macro_31!()