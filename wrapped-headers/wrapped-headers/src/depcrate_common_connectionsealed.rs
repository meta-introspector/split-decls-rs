// Generated macro for sealed (module)
macro_rules! Depcrate_common_connectionsealed {
() => {
// Module: crate::common::connection
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { use http :: HeaderName ; pub trait AsConnectionOption : Sealed { fn as_connection_option (& self) -> & str ; } pub trait Sealed { } impl AsConnectionOption for & str { fn as_connection_option (& self) -> & str { self } } impl Sealed for & str { } impl AsConnectionOption for & HeaderName { fn as_connection_option (& self) -> & str { self . as_ref () } } impl Sealed for & HeaderName { } impl AsConnectionOption for HeaderName { fn as_connection_option (& self) -> & str { self . as_ref () } } impl Sealed for HeaderName { } }
};
}
