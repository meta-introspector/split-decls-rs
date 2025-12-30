// Generated macro for dist (module)
macro_rules! Depcrate_core_builder_testsdist {
() => {
// Module: crate::core::builder::tests
// Provides: {"dist"}
// Dependencies: {}
mod dist { use pretty_assertions :: assert_eq ; use super :: { Config , TEST_TRIPLE_1 , TEST_TRIPLE_2 , TEST_TRIPLE_3 , first , run_build } ; use crate :: Flags ; use crate :: core :: builder :: * ; fn configure (host : & [& str] , target : & [& str]) -> Config { Config { stage : 2 , .. super :: configure ("dist" , host , target) } } # [test] fn llvm_out_behaviour () { let mut config = configure (& [TEST_TRIPLE_1] , & [TEST_TRIPLE_2]) ; config . llvm_from_ci = true ; let build = Build :: new (config . clone ()) ; let target = TargetSelection :: from_user (TEST_TRIPLE_1) ; assert ! (build . llvm_out (target) . ends_with ("ci-llvm")) ; let target = TargetSelection :: from_user (TEST_TRIPLE_2) ; assert ! (build . llvm_out (target) . ends_with ("llvm")) ; config . llvm_from_ci = false ; let build = Build :: new (config . clone ()) ; let target = TargetSelection :: from_user (TEST_TRIPLE_1) ; assert ! (build . llvm_out (target) . ends_with ("llvm")) ; } }
};
}
