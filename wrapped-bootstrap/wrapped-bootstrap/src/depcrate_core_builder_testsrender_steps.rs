// Generated macro for render_steps (function)
macro_rules! Depcrate_core_builder_testsrender_steps {
() => {
// Module: crate::core::builder::tests
// Provides: {"render_steps"}
// Dependencies: {}
# [doc = " Renders the executed bootstrap steps for usage in snapshot tests with insta."] # [doc = " Only renders certain important steps."] # [doc = " Each value in `steps` should be a tuple of (Step, step output)."] # [doc = ""] # [doc = " The arrow in the rendered output (`X -> Y`) means `X builds Y`."] # [doc = " This is similar to the output printed by bootstrap to stdout, but here it is"] # [doc = " generated purely for the purpose of tests."] fn render_steps (steps : & [ExecutedStep] , config : RenderConfig) -> String { steps . iter () . filter_map (| step | { use std :: fmt :: Write ; let Some (metadata) = & step . metadata else { return None ; } ; Some (render_metadata (& metadata , & config)) }) . collect :: < Vec < _ > > () . join ("\n") }
};
}
