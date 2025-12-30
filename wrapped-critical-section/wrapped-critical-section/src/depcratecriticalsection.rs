// Generated macro for CriticalSection (struct)
macro_rules! DepcrateCriticalSection {
() => {
// Module: crate
// Provides: {"CriticalSection"}
// Dependencies: {}
# [doc = " Critical section token."] # [doc = ""] # [doc = " An instance of this type indicates that the current thread is executing code within a critical"] # [doc = " section."] # [derive (Clone , Copy , Debug)] pub struct CriticalSection < 'cs > { _private : PhantomData < & 'cs () > , _not_send_sync : PhantomData < * mut () > , }
};
}
