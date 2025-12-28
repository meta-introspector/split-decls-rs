macro_rules! FIXUP_DUMMY_RANGE {
    () => {
        const FIXUP_DUMMY_RANGE : TextRange = TextRange :: empty (TextSize :: new (0)) ;
    };
}

FIXUP_DUMMY_RANGE!();