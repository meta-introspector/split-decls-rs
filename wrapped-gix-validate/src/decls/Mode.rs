macro_rules! Mode {
    () => {
        # [derive (Eq , PartialEq)] pub (crate) enum Mode { Sanitize , Validate , }
    };
}

Mode!();