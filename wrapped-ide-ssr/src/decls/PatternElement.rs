macro_rules! deps {
    () => {
        Token!();
        Placeholder!();
    };
}

macro_rules! PatternElement {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) enum PatternElement { Token (Token) , Placeholder (Placeholder) , }
    };
}

PatternElement!()