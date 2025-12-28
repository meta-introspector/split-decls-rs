macro_rules! deps {
    () => {
        Registry!();
        JobRef!();
        HeapJob!();
    };
}

macro_rules! spawn_job {
    () => {
        deps!();
        unsafe fn spawn_job < F > (func : F , registry : & Arc < Registry >) -> JobRef where F : FnOnce () + Send + 'static , { registry . increment_terminate_count () ; HeapJob :: new ({ let registry = Arc :: clone (registry) ; move | | { registry . catch_unwind (func) ; registry . terminate () ; } }) . into_static_job_ref () }
    };
}

spawn_job!()