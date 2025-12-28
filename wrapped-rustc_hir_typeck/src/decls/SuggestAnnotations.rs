macro_rules! deps {
    () => {
        SuggestAnnotation!();
    };
}

macro_rules! SuggestAnnotations {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct SuggestAnnotations { pub suggestions : Vec < SuggestAnnotation > , }
    };
}

SuggestAnnotations!()