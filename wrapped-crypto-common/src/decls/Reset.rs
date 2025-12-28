macro_rules! Reset {
    () => {
        # [doc = " Resettable types."] pub trait Reset { # [doc = " Reset state to its initial value."] fn reset (& mut self) ; }
    };
}

Reset!()