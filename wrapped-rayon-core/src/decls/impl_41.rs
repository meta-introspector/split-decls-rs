macro_rules! deps {
    () => {
        JobRef!();
        HeapJob!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < BODY > HeapJob < BODY > where BODY : FnOnce () + Send , { pub (super) fn new (job : BODY) -> Box < Self > { Box :: new (HeapJob { job }) } # [doc = " Creates a `JobRef` from this job -- note that this hides all"] # [doc = " lifetimes, so it is up to you to ensure that this JobRef"] # [doc = " doesn't outlive any data that it closes over."] pub (super) unsafe fn into_job_ref (self : Box < Self >) -> JobRef { unsafe { JobRef :: new (Box :: into_raw (self)) } } # [doc = " Creates a static `JobRef` from this job."] pub (super) fn into_static_job_ref (self : Box < Self >) -> JobRef where BODY : 'static , { unsafe { self . into_job_ref () } } }
    };
}

impl_41!()