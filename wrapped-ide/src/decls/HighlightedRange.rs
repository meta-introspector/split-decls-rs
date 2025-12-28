macro_rules! HighlightedRange {
    () => {
        # [derive (PartialEq , Eq , Hash)] pub struct HighlightedRange { pub range : TextRange , pub category : ReferenceCategory , }
    };
}

HighlightedRange!()