macro_rules! deps {
    () => {
        Visitor!();
        Field!();
        DirectivesUnique!();
        VisitorContext!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < 'a > Visitor < 'a > for DirectivesUnique { fn enter_operation_definition (& mut self , ctx : & mut VisitorContext < 'a > , _name : Option < & 'a Name > , operation_definition : & 'a Positioned < OperationDefinition > ,) { check_duplicate_directive (ctx , & operation_definition . node . directives) ; } fn enter_fragment_definition (& mut self , ctx : & mut VisitorContext < 'a > , _name : & 'a Name , fragment_definition : & 'a Positioned < FragmentDefinition > ,) { check_duplicate_directive (ctx , & fragment_definition . node . directives) ; } fn enter_variable_definition (& mut self , ctx : & mut VisitorContext < 'a > , variable_definition : & 'a Positioned < VariableDefinition > ,) { check_duplicate_directive (ctx , & variable_definition . node . directives) ; } fn enter_field (& mut self , ctx : & mut VisitorContext < 'a > , field : & 'a Positioned < Field >) { check_duplicate_directive (ctx , & field . node . directives) ; } fn enter_fragment_spread (& mut self , ctx : & mut VisitorContext < 'a > , fragment_spread : & 'a Positioned < FragmentSpread > ,) { check_duplicate_directive (ctx , & fragment_spread . node . directives) ; } fn enter_inline_fragment (& mut self , ctx : & mut VisitorContext < 'a > , inline_fragment : & 'a Positioned < InlineFragment > ,) { check_duplicate_directive (ctx , & inline_fragment . node . directives) ; } }
    };
}

impl_184!()