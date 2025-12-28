macro_rules! Empty {
    () => {
        # [doc = " Reader for the [`empty()`] function."] # [must_use = "readers do nothing unless polled"] pub struct Empty { _priv : () , }
    };
}

Empty!()