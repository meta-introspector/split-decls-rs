macro_rules! deps {
    () => {
        MessageLevel!();
    };
}

macro_rules! level_to_style {
    () => {
        deps!();
        fn level_to_style (level : MessageLevel) -> Style { use MessageLevel :: * ; Style :: default () . fg (Color :: Black) . add_modifier (Modifier :: BOLD) . bg (match level { Info => Color :: White , Failure => Color :: Red , Success => Color :: Green , }) }
    };
}

level_to_style!();