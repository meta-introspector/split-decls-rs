macro_rules! deps {
    () => {
        Placeholder!();
    };
}

macro_rules! ParsedRule {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct ParsedRule { pub (crate) placeholders_by_stand_in : FxHashMap < SmolStr , Placeholder > , pub (crate) pattern : SyntaxNode , pub (crate) template : Option < SyntaxNode > , }
    };
}

ParsedRule!()