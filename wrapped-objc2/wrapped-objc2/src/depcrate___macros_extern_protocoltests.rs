// Generated macro for tests (module)
macro_rules! Depcrate___macros_extern_protocoltests {
() => {
// Module: crate::__macros::extern_protocol
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: ProtocolType ; # [test] fn explicit_name () { extern_protocol ! (# [allow (clippy :: missing_safety_doc)] # [name = "NSObject"] unsafe trait Foo { }) ; let proto = < dyn Foo > :: protocol () . unwrap () ; assert_eq ! (proto . name () . to_str () . unwrap () , "NSObject") ; assert_eq ! (< dyn Foo >:: NAME , "NSObject") ; } }
};
}
