// Generated macro for tests (module)
macro_rules! Depcrate_basictests {
() => {
// Module: crate::basic
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn basic () { let ctx = BasicClient { realm : "WallyWorld" . into () , } ; assert_eq ! (ctx . respond ("Aladdin" , "open sesame") , "Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==") ; let ctx = BasicClient { realm : "foo" . into () , } ; assert_eq ! (ctx . respond ("test" , "123\u{A3}") , "Basic dGVzdDoxMjPCow==") ; } }
};
}
