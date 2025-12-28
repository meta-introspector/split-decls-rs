macro_rules! deps {
    () => {
        Visitor!();
        VariablesAreInputTypes!();
        VisitorContext!();
    };
}

macro_rules! impl_266 {
    () => {
        deps!();
        impl < 'a > Visitor < 'a > for VariablesAreInputTypes { fn enter_variable_definition (& mut self , ctx : & mut VisitorContext < 'a > , variable_definition : & 'a Positioned < VariableDefinition > ,) { if let Some (ty) = ctx . registry . concrete_type_by_parsed_type (& variable_definition . node . var_type . node) { if ! ty . is_input () { ctx . report_error (vec ! [variable_definition . pos] , format ! ("Variable \"{}\" cannot be of non-input type \"{}\"" , variable_definition . node . name . node , ty . name ()) ,) ; } } } }
    };
}

impl_266!();