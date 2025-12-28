macro_rules! Visible {
    () => {
        # [derive (Debug)] pub (crate) enum Visible { Yes , Editable , No , }
    };
}

Visible!();