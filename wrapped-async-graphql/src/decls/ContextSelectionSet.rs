macro_rules! deps {
    () => {
        ContextBase!();
        Context!();
    };
}

macro_rules! ContextSelectionSet {
    () => {
        deps!();
        # [doc = " Context for `SelectionSet`"] pub type ContextSelectionSet < 'a > = ContextBase < 'a , & 'a Positioned < SelectionSet > > ;
    };
}

ContextSelectionSet!()