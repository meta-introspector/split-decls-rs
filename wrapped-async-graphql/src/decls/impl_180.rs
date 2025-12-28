macro_rules! deps {
    () => {
        QueryPathNode!();
        QueryPathSegment!();
        VisitorContext!();
        DefaultValuesOfCorrectType!();
        Visitor!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < 'a > Visitor < 'a > for DefaultValuesOfCorrectType { fn enter_variable_definition (& mut self , ctx : & mut VisitorContext < 'a > , variable_definition : & 'a Positioned < VariableDefinition > ,) { if let BaseType :: Named (vtype_name) = & variable_definition . node . var_type . node . base { if ! ctx . registry . types . contains_key (vtype_name . as_str ()) { ctx . report_error (vec ! [variable_definition . pos] , format ! (r#"Unknown type "{}""# , vtype_name) ,) ; return ; } } if let Some (value) = & variable_definition . node . default_value { if let Some (reason) = is_valid_input_value (ctx . registry , & variable_definition . node . var_type . to_string () , & value . node , QueryPathNode { parent : None , segment : QueryPathSegment :: Name (& variable_definition . node . name . node) , } ,) { ctx . report_error (vec ! [variable_definition . pos] , format ! ("Invalid default value for argument: {}" , reason) ,) } } } }
    };
}

impl_180!();