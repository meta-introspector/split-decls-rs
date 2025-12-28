macro_rules! deps {
    () => {
        Placeholder!();
        ParsedRule!();
    };
}

macro_rules! RuleBuilder {
    () => {
        deps!();
        struct RuleBuilder { placeholders_by_stand_in : FxHashMap < SmolStr , Placeholder > , rules : Vec < ParsedRule > , }
    };
}

RuleBuilder!()