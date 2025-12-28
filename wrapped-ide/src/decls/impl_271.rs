macro_rules! deps {
    () => {
        InlayHintLabelPart!();
        InlayHintLabel!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl From < String > for InlayHintLabel { fn from (s : String) -> Self { Self { parts : smallvec ! [InlayHintLabelPart { text : s , linked_location : None , tooltip : None }] , } } }
    };
}

impl_271!()