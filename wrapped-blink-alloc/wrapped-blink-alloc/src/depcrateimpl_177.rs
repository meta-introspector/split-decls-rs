// Generated macro for impl_177 (impl)
macro_rules! Depcrateimpl_177 {
() => {
// Module: crate
// Provides: {"impl_177"}
// Dependencies: {}
impl < T > ResultExt < T > for Result < T , core :: convert :: Infallible > { # [inline] fn safe_ok (self) -> T { match self { Ok (value) => value , Err (never) => match never { } , } } }
};
}
