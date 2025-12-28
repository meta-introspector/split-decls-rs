macro_rules! deps {
    () => {
        InlayHintLabel!();
        InlayHintLabelPart!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl From < & str > for InlayHintLabel { fn from (s : & str) -> Self { Self { parts : smallvec ! [InlayHintLabelPart { text : s . into () , linked_location : None , tooltip : None }] , } } }
    };
}

impl_272!();