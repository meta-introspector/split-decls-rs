macro_rules! deps {
    () => {
        VisitorContext!();
        Visitor!();
    };
}

macro_rules! visit_variable_definitions {
    () => {
        deps!();
        fn visit_variable_definitions < 'a , V : Visitor < 'a > > (v : & mut V , ctx : & mut VisitorContext < 'a > , variable_definitions : & 'a [Positioned < VariableDefinition >] ,) { for d in variable_definitions { v . enter_variable_definition (ctx , d) ; v . exit_variable_definition (ctx , d) ; } }
    };
}

visit_variable_definitions!()