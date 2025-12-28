macro_rules! deps {
    () => {
        Scope!();
    };
}

macro_rules! NoUndefinedVariables {
    () => {
        deps!();
        # [derive (Default)] pub struct NoUndefinedVariables < 'a > { defined_variables : HashMap < Option < & 'a str > , (Pos , HashSet < & 'a str >) > , used_variables : HashMap < Scope < 'a > , HashMap < & 'a str , Pos > > , current_scope : Option < Scope < 'a > > , spreads : HashMap < Scope < 'a > , Vec < & 'a str > > , }
    };
}

NoUndefinedVariables!()