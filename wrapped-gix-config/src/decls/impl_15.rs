macro_rules! deps {
    () => {
        Whitespace!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Default for Whitespace < '_ > { fn default () -> Self { Whitespace { pre_key : Some (b"\t" . as_bstr () . into ()) , pre_sep : Some (b" " . as_bstr () . into ()) , post_sep : Some (b" " . as_bstr () . into ()) , } } }
    };
}

impl_15!();