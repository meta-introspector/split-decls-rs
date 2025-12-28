macro_rules! deps {
    () => {
        HwCounterType!();
        Counter!();
        HwCounterRead!();
    };
}

macro_rules! hw {
    () => {
        deps!();
        # [cfg (not (all (target_arch = "x86_64" , target_os = "linux" , not (target_env = "ohos"))))] mod hw { use std :: error :: Error ; pub (super) enum Counter { } impl Counter { pub (super) fn new (model : & CpuModel , _ : super :: HwCounterType ,) -> Result < Self , Box < dyn Error + Send + Sync > > { match * model { } } } impl super :: HwCounterRead for Counter { type Output = u64 ; # [inline] fn read (& self) -> u64 { match * self { } } } impl super :: HwCounterRead for (& Counter , & Counter) { type Output = (u64 , u64) ; # [inline] fn read (& self) -> (u64 , u64) { match * self . 0 { } } } pub (super) enum CpuModel { } impl CpuModel { pub (super) fn detect () -> Result < Self , Box < dyn Error + Send + Sync > > { if false { really_warn ! ("unsupported; {}" , super :: BUG_REPORT_MSG) ; } let mut msg = String :: new () ; let mut add_error = | s | { if ! msg . is_empty () { msg += "; " ; } msg += s ; } ; if cfg ! (not (target_arch = "x86_64")) { add_error ("only supported architecture is x86_64") ; } if cfg ! (not (target_os = "linux")) { add_error ("only supported OS is Linux") ; } if cfg ! (target_env = "ohos") { add_error ("unsupported OHOS environment") ; } Err (msg . into ()) } } }
    };
}

hw!();