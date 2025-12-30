// Generated macro for impl_900 (impl)
macro_rules! Depcrate_core_builder_testsimpl_900 {
() => {
// Module: crate::core::builder::tests
// Provides: {"impl_900"}
// Dependencies: {}
impl ConfigBuilder { fn run (self) -> Cache { let config = self . create_config () ; let kind = config . cmd . kind () ; let build = Build :: new (config) ; let builder = Builder :: new (& build) ; builder . run_step_descriptions (& Builder :: get_step_descriptions (kind) , & builder . paths) ; builder . cache } fn get_steps (self) -> ExecutedSteps { let cache = self . run () ; ExecutedSteps { steps : cache . into_executed_steps () } } fn render_steps (self) -> String { self . get_steps () . render () } }
};
}
