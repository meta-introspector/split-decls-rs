// Generated macro for PanicHandler (type)
macro_rules! DepcratePanicHandler {
() => {
// Module: crate
// Provides: {"PanicHandler"}
// Dependencies: {}
# [doc = " The type for a panic-handling closure. Note that this same closure"] # [doc = " may be invoked multiple times in parallel."] type PanicHandler = dyn Fn (Box < dyn Any + Send >) + Send + Sync ;
};
}
