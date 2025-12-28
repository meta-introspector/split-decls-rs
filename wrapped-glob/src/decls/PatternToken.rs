macro_rules! deps {
    () => {
        CharSpecifier!();
    };
}

macro_rules! PatternToken {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] enum PatternToken { Char (char) , AnyChar , AnySequence , AnyRecursiveSequence , AnyWithin (Vec < CharSpecifier >) , AnyExcept (Vec < CharSpecifier >) , }
    };
}

PatternToken!();