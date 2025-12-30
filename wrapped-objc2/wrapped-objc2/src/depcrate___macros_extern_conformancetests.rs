// Generated macro for tests (module)
macro_rules! Depcrate___macros_extern_conformancetests {
() => {
// Module: crate::__macros::extern_conformance
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [allow (dead_code)] mod tests { use crate :: runtime :: NSObject ; use crate :: { extern_class , extern_protocol } ; extern_class ! (# [unsafe (super (NSObject))] # [name = "NSObject"] struct OldSyntax ;) ; extern_protocol ! (# [name = "NSObjectProtocol"] # [allow (clippy :: missing_safety_doc)] unsafe trait Protocol { }) ; unsafe impl Protocol for OldSyntax { } }
};
}
