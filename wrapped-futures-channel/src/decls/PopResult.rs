macro_rules! PopResult {
    () => {
        # [doc = " A result of the `pop` function."] pub (super) enum PopResult < T > { # [doc = " Some data has been popped"] Data (T) , # [doc = " The queue is empty"] Empty , # [doc = " The queue is in an inconsistent state. Popping data should succeed, but"] # [doc = " some pushers have yet to make enough progress in order allow a pop to"] # [doc = " succeed. It is recommended that a pop() occur \"in the near future\" in"] # [doc = " order to see if the sender has made progress or not"] Inconsistent , }
    };
}

PopResult!();