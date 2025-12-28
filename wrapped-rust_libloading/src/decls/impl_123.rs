macro_rules! deps {
    () => {
        ErrorModeGuard!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl ErrorModeGuard { # [allow (clippy :: if_same_then_else)] fn new () -> Option < ErrorModeGuard > { unsafe { let mut previous_mode = 0 ; if SetThreadErrorMode (SEM_FAILCRITICALERRORS , & mut previous_mode) == 0 { None } else if previous_mode == SEM_FAILCRITICALERRORS { None } else { Some (ErrorModeGuard (previous_mode)) } } } }
    };
}

impl_123!()