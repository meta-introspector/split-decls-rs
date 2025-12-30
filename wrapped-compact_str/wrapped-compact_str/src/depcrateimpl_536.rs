// Generated macro for impl_536 (impl)
macro_rules! Depcrateimpl_536 {
() => {
// Module: crate
// Provides: {"impl_536"}
// Dependencies: {}
impl < T , E : fmt :: Display > UnwrapWithMsg for Result < T , E > { type T = T ; # [inline (always)] # [track_caller] fn unwrap_with_msg (self) -> T { match self { Ok (value) => value , Err (err) => unwrap_with_msg_fail (err) , } } }
};
}
