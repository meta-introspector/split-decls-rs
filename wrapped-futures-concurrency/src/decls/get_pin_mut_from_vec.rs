macro_rules! get_pin_mut_from_vec {
    () => {
        # [cfg (feature = "alloc")] pub (crate) fn get_pin_mut_from_vec < T , I > (slice : Pin < & mut Vec < T > > , index : I ,) -> Option < Pin < & mut I :: Output > > where I : SliceIndex < [T] > , { unsafe { slice . get_unchecked_mut () . get_mut (index) . map (| x | Pin :: new_unchecked (x)) } }
    };
}

get_pin_mut_from_vec!();