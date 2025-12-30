// Generated macro for private_try_future (module)
macro_rules! Depcrate_futureprivate_try_future {
() => {
// Module: crate::future
// Provides: {"private_try_future"}
// Dependencies: {}
mod private_try_future { use super :: Future ; pub trait Sealed { } impl < F , T , E > Sealed for F where F : ? Sized + Future < Output = Result < T , E > > { } }
};
}
