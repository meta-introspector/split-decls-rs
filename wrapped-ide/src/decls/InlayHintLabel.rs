macro_rules! deps {
    () => {
        InlayHintLabelPart!();
    };
}

macro_rules! InlayHintLabel {
    () => {
        deps!();
        # [derive (Default , Hash , UpmapFromRaFixture)] pub struct InlayHintLabel { pub parts : SmallVec < [InlayHintLabelPart ; 1] > , }
    };
}

InlayHintLabel!();