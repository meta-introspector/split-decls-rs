macro_rules! CaptureBy {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum CaptureBy { # [doc = " `move |x| y + x`."] Value , # [doc = " `move` keyword was not specified."] Ref , }
    };
}

CaptureBy!();