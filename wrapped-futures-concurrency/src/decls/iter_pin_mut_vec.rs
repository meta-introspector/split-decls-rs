macro_rules! iter_pin_mut_vec {
    () => {
        # [cfg (feature = "alloc")] pub (crate) fn iter_pin_mut_vec < T > (slice : Pin < & mut Vec < T > >) -> impl Iterator < Item = Pin < & mut T > > { unsafe { slice . get_unchecked_mut () } . iter_mut () . map (| t | unsafe { Pin :: new_unchecked (t) }) }
    };
}

iter_pin_mut_vec!()