macro_rules! deps {
    () => {
        VariableInAllowedPosition!();
        MetaTypeName!();
        VisitorContext!();
        Scope!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < 'a > VariableInAllowedPosition < 'a > { fn collect_incorrect_usages (& self , from : & Scope < 'a > , var_defs : & [& 'a Positioned < VariableDefinition >] , ctx : & mut VisitorContext < 'a > , visited : & mut HashSet < Scope < 'a > > ,) { if visited . contains (from) { return ; } visited . insert (* from) ; if let Some (usages) = self . variable_usages . get (from) { for (var_name , usage_pos , var_type) in usages { if let Some (def) = var_defs . iter () . find (| def | def . node . name . node == * var_name) { let expected_type = if def . node . var_type . node . nullable && def . node . default_value . is_some () { format ! ("{}!" , def . node . var_type . node) } else { def . node . var_type . node . to_string () } ; if ! var_type . is_subtype (& MetaTypeName :: create (& expected_type)) { ctx . report_error (vec ! [def . pos , * usage_pos] , format ! ("Variable \"{}\" of type \"{}\" used in position expecting type \"{}\"" , var_name , var_type , expected_type) ,) ; } } } } if let Some (spreads) = self . spreads . get (from) { for spread in spreads { self . collect_incorrect_usages (& Scope :: Fragment (spread) , var_defs , ctx , visited) ; } } } }
    };
}

impl_270!();