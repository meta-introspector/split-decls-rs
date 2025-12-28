macro_rules! WalkError {
    () => {
        # [doc = " Error from iterating on files."] pub type WalkError = walkdir :: Error ;
    };
}

WalkError!();