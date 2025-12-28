macro_rules! likely {
    () => {
        # [cfg (not (feature = "nightly"))] # [inline (always)] pub (crate) fn likely (b : bool) -> bool { if b { true } else { cold_path () ; false } }
    };
}

likely!()