macro_rules! RenameError {
    () => {
        # [derive (Debug)] pub struct RenameError (pub String) ;
    };
}

RenameError!();