macro_rules! deps {
    () => {
        MismatchedLifetimeSyntaxesSuggestion!();
        LifetimeSyntaxCategories!();
    };
}

macro_rules! MismatchedLifetimeSyntaxes {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct MismatchedLifetimeSyntaxes { pub inputs : LifetimeSyntaxCategories < Vec < Span > > , pub outputs : LifetimeSyntaxCategories < Vec < Span > > , pub suggestions : Vec < MismatchedLifetimeSyntaxesSuggestion > , }
    };
}

MismatchedLifetimeSyntaxes!()