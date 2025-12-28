macro_rules! MinReset {
    () => {
        # [doc = " Resets state, without performing expensive ops (e.g. zeroing buffer)"] # [doc = ""] # [doc = " Note that not zeroing buffer can lead to security issues when dealing with untrusted input."] pub struct MinReset ;
    };
}

MinReset!()