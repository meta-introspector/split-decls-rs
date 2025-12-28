macro_rules! offset_from {
    () => {
        # [inline] unsafe fn offset_from < T > (to : * const T , from : * const T) -> usize { to . offset_from (from) as usize }
    };
}

offset_from!()