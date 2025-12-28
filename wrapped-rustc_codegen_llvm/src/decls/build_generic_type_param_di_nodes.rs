macro_rules! deps {
    () => {
        CodegenCx!();
        SmallVec!();
    };
}

macro_rules! build_generic_type_param_di_nodes {
    () => {
        deps!();
        # [doc = " Computes the type parameters for a type, if any, for the given metadata."] fn build_generic_type_param_di_nodes < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , ty : Ty < 'tcx > ,) -> SmallVec < Option < & 'll DIType > > { if let ty :: Adt (def , args) = * ty . kind () { if args . types () . next () . is_some () { let generics = cx . tcx . generics_of (def . did ()) ; let names = get_parameter_names (cx , generics) ; let template_params : SmallVec < _ > = iter :: zip (args , names) . filter_map (| (kind , name) | { kind . as_type () . map (| ty | { let actual_type = cx . tcx . normalize_erasing_regions (cx . typing_env () , ty) ; let actual_type_di_node = type_di_node (cx , actual_type) ; Some (cx . create_template_type_parameter (name . as_str () , actual_type_di_node)) }) }) . collect () ; return template_params ; } } return smallvec ! [] ; fn get_parameter_names (cx : & CodegenCx < '_ , '_ > , generics : & ty :: Generics) -> Vec < Symbol > { let mut names = generics . parent . map_or_else (Vec :: new , | def_id | get_parameter_names (cx , cx . tcx . generics_of (def_id))) ; names . extend (generics . own_params . iter () . map (| param | param . name)) ; names } }
    };
}

build_generic_type_param_di_nodes!();