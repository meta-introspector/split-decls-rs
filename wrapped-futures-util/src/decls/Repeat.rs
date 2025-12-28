macro_rules! Repeat {
    () => {
        # [doc = " Reader for the [`repeat()`] function."] # [must_use = "readers do nothing unless polled"] pub struct Repeat { byte : u8 , }
    };
}

Repeat!();