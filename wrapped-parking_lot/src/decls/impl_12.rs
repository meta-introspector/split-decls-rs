macro_rules! deps {
    () => {
        AtomicElisionExt!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [cfg (all (feature = "hardware-lock-elision" , not (miri) , any (target_arch = "x86" , target_arch = "x86_64")))] impl AtomicElisionExt for AtomicUsize { type IntType = usize ; # [inline] fn elision_compare_exchange_acquire (& self , current : usize , new : usize) -> Result < usize , usize > { unsafe { use core :: arch :: asm ; let prev : usize ; # [cfg (target_pointer_width = "32")] asm ! ("xacquire" , "lock" , "cmpxchg [{:e}], {:e}" , in (reg) self , in (reg) new , inout ("eax") current => prev ,) ; # [cfg (target_pointer_width = "64")] asm ! ("xacquire" , "lock" , "cmpxchg [{}], {}" , in (reg) self , in (reg) new , inout ("rax") current => prev ,) ; if prev == current { Ok (prev) } else { Err (prev) } } } # [inline] fn elision_fetch_sub_release (& self , val : usize) -> usize { unsafe { use core :: arch :: asm ; let prev : usize ; # [cfg (target_pointer_width = "32")] asm ! ("xrelease" , "lock" , "xadd [{:e}], {:e}" , in (reg) self , inout (reg) val . wrapping_neg () => prev ,) ; # [cfg (target_pointer_width = "64")] asm ! ("xrelease" , "lock" , "xadd [{}], {}" , in (reg) self , inout (reg) val . wrapping_neg () => prev ,) ; prev } } }
    };
}

impl_12!()