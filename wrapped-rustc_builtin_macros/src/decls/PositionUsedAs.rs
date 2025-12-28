macro_rules! PositionUsedAs {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq)] enum PositionUsedAs { Placeholder (Option < Span >) , Precision , Width , }
    };
}

PositionUsedAs!();