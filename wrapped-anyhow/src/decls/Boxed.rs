macro_rules! Boxed {
    () => {
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] pub struct Boxed ;
    };
}

Boxed!();