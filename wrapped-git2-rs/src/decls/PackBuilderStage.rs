macro_rules! deps {
    () => {
        PackBuilder!();
    };
}

macro_rules! PackBuilderStage {
    () => {
        deps!();
        # [derive (PartialEq , Eq , Clone , Debug , Copy)] # [doc = " Stages that are reported by the `PackBuilder` progress callback."] pub enum PackBuilderStage { # [doc = " Adding objects to the pack"] AddingObjects , # [doc = " Deltafication of the pack"] Deltafication , }
    };
}

PackBuilderStage!();