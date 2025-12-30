// Generated macro for sealed (module)
macro_rules! Depcrate_runnablesealed {
() => {
// Module: crate::runnable
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { use super :: * ; pub trait Sealed < M > { } impl < M , F > Sealed < M > for F where F : Fn (Runnable < M >) { } impl < M , F > Sealed < M > for WithInfo < F > where F : Fn (Runnable < M > , ScheduleInfo) { } }
};
}
