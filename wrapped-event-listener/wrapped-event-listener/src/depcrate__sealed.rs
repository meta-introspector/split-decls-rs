// Generated macro for __sealed (module)
macro_rules! Depcrate__sealed {
() => {
// Module: crate
// Provides: {"__sealed"}
// Dependencies: {}
# [doc (hidden)] mod __sealed { use super :: { EventListener , __private :: StackListener } ; pub trait Sealed { } impl < T > Sealed for EventListener < T > { } impl < T > Sealed for StackListener < '_ , '_ , T > { } }
};
}
