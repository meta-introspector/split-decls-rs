macro_rules! Empty {
    () => {
        # [doc = " Reader for the [`empty()`] function."] pub struct Empty { _private : () , }
    };
}

Empty!()