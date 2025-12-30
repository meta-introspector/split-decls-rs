// Generated macro for impl_827 (impl)
macro_rules! Depcrate_packbuilderimpl_827 {
() => {
// Module: crate::packbuilder
// Provides: {"impl_827"}
// Dependencies: {}
impl Binding for PackBuilderStage { type Raw = raw :: git_packbuilder_stage_t ; unsafe fn from_raw (raw : raw :: git_packbuilder_stage_t) -> PackBuilderStage { match raw { raw :: GIT_PACKBUILDER_ADDING_OBJECTS => PackBuilderStage :: AddingObjects , raw :: GIT_PACKBUILDER_DELTAFICATION => PackBuilderStage :: Deltafication , _ => panic ! ("Unknown git diff binary kind") , } } fn raw (& self) -> raw :: git_packbuilder_stage_t { match * self { PackBuilderStage :: AddingObjects => raw :: GIT_PACKBUILDER_ADDING_OBJECTS , PackBuilderStage :: Deltafication => raw :: GIT_PACKBUILDER_DELTAFICATION , } } }
};
}
