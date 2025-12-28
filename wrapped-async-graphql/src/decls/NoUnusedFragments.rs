macro_rules! deps {
    () => {
        Scope!();
    };
}

macro_rules! NoUnusedFragments {
    () => {
        deps!();
        # [derive (Default)] pub struct NoUnusedFragments < 'a > { spreads : HashMap < Scope < 'a > , Vec < & 'a str > > , defined_fragments : HashSet < (& 'a str , Pos) > , current_scope : Option < Scope < 'a > > , }
    };
}

NoUnusedFragments!()