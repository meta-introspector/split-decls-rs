// Generated macro for ServiceFn (struct)
macro_rules! Depcrate_service_utilServiceFn {
() => {
// Module: crate::service::util
// Provides: {"ServiceFn"}
// Dependencies: {}
# [doc = " Service returned by [`service_fn`]"] pub struct ServiceFn < F , R > { f : F , _req : PhantomData < fn (R) > , }
};
}
