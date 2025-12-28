macro_rules! deps {
    () => {
        IWeakReference_Vtbl!();
        IWeakReferenceSource_Vtbl!();
        RefCount!();
    };
}

macro_rules! TearOff {
    () => {
        deps!();
        # [repr (C)] struct TearOff { strong_vtable : * const IWeakReferenceSource_Vtbl , weak_vtable : * const IWeakReference_Vtbl , object : * mut c_void , strong_count : RefCount , weak_count : RefCount , }
    };
}

TearOff!();