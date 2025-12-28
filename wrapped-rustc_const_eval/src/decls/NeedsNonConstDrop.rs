macro_rules! NeedsNonConstDrop {
    () => {
        # [doc = " Constant containing an ADT that implements non-const `Drop`."] # [doc = " This must be ruled out because we cannot run `Drop` during compile-time."] pub struct NeedsNonConstDrop ;
    };
}

NeedsNonConstDrop!();