macro_rules! unlikely {
    () => {
        # [cfg (not (feature = "nightly"))] # [inline (always)] pub (crate) fn unlikely (b : bool) -> bool { if b { cold_path () ; true } else { false } }
    };
}

unlikely!();