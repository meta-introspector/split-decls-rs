macro_rules! deps {
    () => {
        Context!();
        ContextBase!();
    };
}

macro_rules! ContextSelectionSet {
    () => {
        deps!();
        # [doc = " Context for `SelectionSet`"] pub type ContextSelectionSet < 'a > = ContextBase < 'a , & 'a Positioned < SelectionSet > > ;
    };
}

ContextSelectionSet!();