macro_rules! SingleArmMatchBegin {
    () => {
        enum SingleArmMatchBegin { WithOpenBracket (Span) , WithoutOpenBracket (Span) , }
    };
}

SingleArmMatchBegin!();