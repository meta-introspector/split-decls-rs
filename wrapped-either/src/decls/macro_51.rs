macro_rules! macro_51 {
    () => {
        impl_specific_ref_and_mut ! (:: std :: ffi :: OsStr , cfg (feature = "std") , doc = "Requires crate feature `std`.") ;
    };
}

macro_51!();