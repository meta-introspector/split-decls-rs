// Generated macro for Being (enum)
macro_rules! Depcrate_validation_test_harnessBeing {
() => {
// Module: crate::validation::test_harness
// Provides: {"Being"}
// Dependencies: {}
# [derive (Interface)] # [graphql (internal , field (name = "name" , ty = "Option<String>" , arg (name = "surname" , ty = "Option<bool>")))] enum Being { Dog (Dog) , Cat (Cat) , Human (Human) , Alien (Alien) , }
};
}
