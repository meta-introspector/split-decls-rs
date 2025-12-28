macro_rules! deps {
    () => {
        Expression!();
        TextElementPosition!();
    };
}

macro_rules! PatternElementPlaceholders {
    () => {
        deps!();
        # [derive (Debug)] enum PatternElementPlaceholders < S > { Placeable (ast :: Expression < S >) , TextElement (usize , usize , usize , TextElementPosition) , }
    };
}

PatternElementPlaceholders!();