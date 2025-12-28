macro_rules! WindowsError {
    () => {
        # [doc = " A Windows API error."] # [derive (Copy , Clone)] pub struct WindowsError (pub (crate) i32) ;
    };
}

WindowsError!();