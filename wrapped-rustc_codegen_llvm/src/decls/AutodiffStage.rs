macro_rules! AutodiffStage {
    () => {
        # [derive (Debug , Eq , PartialEq)] pub (crate) enum AutodiffStage { PreAD , DuringAD , PostAD , }
    };
}

AutodiffStage!()