macro_rules! deps {
    () => {
        Scope!();
        MetaTypeName!();
    };
}

macro_rules! VariableInAllowedPosition {
    () => {
        deps!();
        # [derive (Default)] pub struct VariableInAllowedPosition < 'a > { spreads : HashMap < Scope < 'a > , HashSet < & 'a str > > , variable_usages : HashMap < Scope < 'a > , Vec < (& 'a str , Pos , MetaTypeName < 'a >) > > , variable_defs : HashMap < Scope < 'a > , Vec < & 'a Positioned < VariableDefinition > > > , current_scope : Option < Scope < 'a > > , }
    };
}

VariableInAllowedPosition!();