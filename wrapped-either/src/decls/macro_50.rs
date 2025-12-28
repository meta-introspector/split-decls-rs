macro_rules! macro_50 {
    () => {
        impl_specific_ref_and_mut ! (:: std :: path :: Path , cfg (feature = "std") , doc = "Requires crate feature `std`.") ;
    };
}

macro_50!();