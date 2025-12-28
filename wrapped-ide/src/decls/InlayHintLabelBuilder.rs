macro_rules! deps {
    () => {
        InlayHintLabel!();
        LazyProperty!();
    };
}

macro_rules! InlayHintLabelBuilder {
    () => {
        deps!();
        # [derive (Debug)] struct InlayHintLabelBuilder < 'a > { sema : & 'a Semantics < 'a , RootDatabase > , result : InlayHintLabel , last_part : String , resolve : bool , location : Option < LazyProperty < FileRange > > , }
    };
}

InlayHintLabelBuilder!();