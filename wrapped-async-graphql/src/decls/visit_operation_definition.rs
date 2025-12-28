macro_rules! deps {
    () => {
        Query!();
        Mutation!();
        Visitor!();
        Subscription!();
        Schema!();
        VisitorContext!();
    };
}

macro_rules! visit_operation_definition {
    () => {
        deps!();
        fn visit_operation_definition < 'a , V : Visitor < 'a > > (v : & mut V , ctx : & mut VisitorContext < 'a > , name : Option < & 'a Name > , operation : & 'a Positioned < OperationDefinition > ,) { v . enter_operation_definition (ctx , name , operation) ; let root_name = match & operation . node . ty { OperationType :: Query => Some (& * ctx . registry . query_type) , OperationType :: Mutation => ctx . registry . mutation_type . as_deref () , OperationType :: Subscription => ctx . registry . subscription_type . as_deref () , } ; if let Some (root_name) = root_name { ctx . with_type (Some (& ctx . registry . types [root_name]) , | ctx | { visit_variable_definitions (v , ctx , & operation . node . variable_definitions) ; visit_directives (v , ctx , & operation . node . directives) ; visit_selection_set (v , ctx , & operation . node . selection_set) ; }) ; } else { ctx . report_error (vec ! [operation . pos] , format ! ("Schema is not configured for {}s." , operation . node . ty) ,) ; } v . exit_operation_definition (ctx , name , operation) ; }
    };
}

visit_operation_definition!()