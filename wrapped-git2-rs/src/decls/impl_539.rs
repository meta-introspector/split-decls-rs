macro_rules! deps {
    () => {
        PackBuilderStage!();
        Binding!();
    };
}

macro_rules! impl_539 {
    () => {
        deps!();
        impl Binding for PackBuilderStage { type Raw = raw :: git_packbuilder_stage_t ; unsafe fn from_raw (raw : raw :: git_packbuilder_stage_t) -> PackBuilderStage { match raw { raw :: GIT_PACKBUILDER_ADDING_OBJECTS => PackBuilderStage :: AddingObjects , raw :: GIT_PACKBUILDER_DELTAFICATION => PackBuilderStage :: Deltafication , _ => panic ! ("Unknown git diff binary kind") , } } fn raw (& self) -> raw :: git_packbuilder_stage_t { match * self { PackBuilderStage :: AddingObjects => raw :: GIT_PACKBUILDER_ADDING_OBJECTS , PackBuilderStage :: Deltafication => raw :: GIT_PACKBUILDER_DELTAFICATION , } } }
    };
}

impl_539!();