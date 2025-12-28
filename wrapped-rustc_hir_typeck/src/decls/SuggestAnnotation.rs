macro_rules! SuggestAnnotation {
    () => {
        # [derive (Clone)] pub (crate) enum SuggestAnnotation { Unit (Span) , Path (Span) , Local (Span) , Turbo (Span , usize , usize) , }
    };
}

SuggestAnnotation!();