macro_rules! IterError {
    () => {
        # [doc = " An error that occurs during iteration over the list."] # [derive (PartialEq , Debug)] pub (crate) enum IterError { # [doc = " A concurrent thread modified the state of the list at the same place that this iterator"] # [doc = " was inspecting. Subsequent iteration will restart from the beginning of the list."] Stalled , }
    };
}

IterError!();