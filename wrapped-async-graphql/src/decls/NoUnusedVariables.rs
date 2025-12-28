macro_rules! deps {
    () => {
        Scope!();
    };
}

macro_rules! NoUnusedVariables {
    () => {
        deps!();
        # [derive (Default)] pub struct NoUnusedVariables < 'a > { defined_variables : HashMap < Option < & 'a str > , HashSet < (& 'a str , Pos) > > , used_variables : HashMap < Scope < 'a > , Vec < & 'a str > > , current_scope : Option < Scope < 'a > > , spreads : HashMap < Scope < 'a > , Vec < & 'a str > > , }
    };
}

NoUnusedVariables!()