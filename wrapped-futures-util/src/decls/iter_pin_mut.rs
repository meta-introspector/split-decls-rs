macro_rules! iter_pin_mut {
    () => {
        pub (crate) fn iter_pin_mut < T > (slice : Pin < & mut [T] >) -> impl Iterator < Item = Pin < & mut T > > { unsafe { slice . get_unchecked_mut () } . iter_mut () . map (| t | unsafe { Pin :: new_unchecked (t) }) }
    };
}

iter_pin_mut!();