macro_rules! deps {
    () => {
        SetupInstance!();
        EnumSetupInstances!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl Iterator for EnumSetupInstances { type Item = Result < SetupInstance , i32 > ; fn next (& mut self) -> Option < Result < SetupInstance , i32 > > { let mut obj = null_mut () ; let err = unsafe { self . 0 . Next (1 , & mut obj , null_mut ()) } ; if err < 0 { return Some (Err (err)) ; } if err == S_FALSE { return None ; } Some (Ok (unsafe { SetupInstance :: from_raw (obj) })) } }
    };
}

impl_153!();