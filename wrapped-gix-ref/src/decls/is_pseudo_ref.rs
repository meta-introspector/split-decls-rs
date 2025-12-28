macro_rules! is_pseudo_ref {
    () => {
        # [doc = " Note that this method is disagreeing with `gix_validate` as it allows dashes '-' for some reason."] # [doc = " Since partial names cannot be created with dashes inside we adjusted this as it's probably unintended or git creates pseudo-refs"] # [doc = " which wouldn't pass its safety checks."] pub (crate) fn is_pseudo_ref (name : & BStr) -> bool { name . bytes () . all (| b | b . is_ascii_uppercase () || b == b'_') }
    };
}

is_pseudo_ref!()