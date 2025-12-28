macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! TypeInfo {
    () => {
        deps!();
        # [derive (Debug)] pub struct TypeInfo < 'db > { # [doc = " The original type of the expression or pattern."] pub original : Type < 'db > , # [doc = " The adjusted type, if an adjustment happened."] pub adjusted : Option < Type < 'db > > , }
    };
}

TypeInfo!();