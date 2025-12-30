// Generated macro for private_try_stream (module)
macro_rules! Depcrate_streamprivate_try_stream {
() => {
// Module: crate::stream
// Provides: {"private_try_stream"}
// Dependencies: {}
mod private_try_stream { use super :: Stream ; pub trait Sealed { } impl < S , T , E > Sealed for S where S : ? Sized + Stream < Item = Result < T , E > > { } }
};
}
