macro_rules! deps {
    () => {
        StackJob!();
        JobResult!();
        Latch!();
        JobRef!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < L , F , R > StackJob < L , F , R > where L : Latch + Sync , F : FnOnce (bool) -> R + Send , R : Send , { pub (super) fn new (func : F , latch : L) -> StackJob < L , F , R > { StackJob { latch , func : UnsafeCell :: new (Some (func)) , result : UnsafeCell :: new (JobResult :: None) , } } pub (super) unsafe fn as_job_ref (& self) -> JobRef { unsafe { JobRef :: new (self) } } pub (super) unsafe fn run_inline (self , stolen : bool) -> R { self . func . into_inner () . unwrap () (stolen) } pub (super) unsafe fn into_result (self) -> R { self . result . into_inner () . into_return_value () } }
    };
}

impl_38!()