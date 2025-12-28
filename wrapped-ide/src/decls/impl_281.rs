macro_rules! deps {
    () => {
        InlayHintLabelBuilder!();
        InlayHintLabel!();
        InlayHintLabelPart!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl InlayHintLabelBuilder < '_ > { fn make_new_part (& mut self) { let text = take (& mut self . last_part) ; if ! text . is_empty () { self . result . parts . push (InlayHintLabelPart { text , linked_location : self . location . take () , tooltip : None , }) ; } } fn finish (mut self) -> InlayHintLabel { self . make_new_part () ; self . result } }
    };
}

impl_281!();