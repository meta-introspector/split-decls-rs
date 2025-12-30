// Generated macro for tests (module)
macro_rules! Depcrate_advicetests {
() => {
// Module: crate::advice
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [cfg (target_os = "linux")] # [test] fn test_is_supported () { use super :: * ; assert ! (Advice :: Normal . is_supported ()) ; assert ! (Advice :: Random . is_supported ()) ; assert ! (Advice :: Sequential . is_supported ()) ; assert ! (Advice :: WillNeed . is_supported ()) ; assert ! (UncheckedAdvice :: DontNeed . is_supported ()) ; } }
};
}
