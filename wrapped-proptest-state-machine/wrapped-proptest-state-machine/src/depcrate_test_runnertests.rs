// Generated macro for tests (module)
macro_rules! Depcrate_test_runnertests {
() => {
// Module: crate::test_runner
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { mod macro_test { # ! [doc = " tests to verify that invocations of all forms of the"] # ! [doc = " `prop_state_machine!` macro compile cleanly, and hygenically,"] # ! [doc = "  as intended."] # [doc = " Note: no imports here, so as to guarantee hygienic macros"] # [doc = " A no-op test. Exists strictly as something to reference"] # [doc = " in the macro invocation."] struct Test ; impl crate :: ReferenceStateMachine for Test { type State = () ; type Transition = () ; fn init_state () -> proptest :: strategy :: BoxedStrategy < Self :: State > { use proptest :: prelude :: * ; Just (()) . boxed () } fn transitions (_ : & Self :: State ,) -> proptest :: strategy :: BoxedStrategy < Self :: Transition > { use proptest :: prelude :: * ; Just (()) . boxed () } fn apply (_ : Self :: State , _ : & Self :: Transition) -> Self :: State { () } } impl crate :: StateMachineTest for Test { type SystemUnderTest = () ; type Reference = Self ; fn init_test (_ : & < Self :: Reference as crate :: ReferenceStateMachine > :: State ,) -> Self :: SystemUnderTest { } fn apply (_ : Self :: SystemUnderTest , _ : & < Self :: Reference as crate :: ReferenceStateMachine > :: State , _ : < Self :: Reference as crate :: ReferenceStateMachine > :: Transition ,) -> Self :: SystemUnderTest { } } prop_state_machine ! { # [test] fn no_config_annotation (sequential 1 .. 2 => Test) ; } prop_state_machine ! { #! [proptest_config (:: proptest :: test_runner :: Config :: default ())] # [test] fn with_config_annotation (sequential 1 .. 2 => Test) ; } } }
};
}
