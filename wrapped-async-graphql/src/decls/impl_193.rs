macro_rules! deps {
    () => {
        VisitorContext!();
        FragmentsOnCompositeTypes!();
        Visitor!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < 'a > Visitor < 'a > for FragmentsOnCompositeTypes { fn enter_fragment_definition (& mut self , ctx : & mut VisitorContext < 'a > , name : & 'a Name , fragment_definition : & 'a Positioned < FragmentDefinition > ,) { if let Some (current_type) = ctx . current_type () { if ! current_type . is_composite () { ctx . report_error (vec ! [fragment_definition . pos] , format ! ("Fragment \"{}\" cannot condition non composite type \"{}\"" , name , fragment_definition . node . type_condition . node . on . node ,) ,) ; } } } fn enter_inline_fragment (& mut self , ctx : & mut VisitorContext < 'a > , inline_fragment : & 'a Positioned < InlineFragment > ,) { if let Some (current_type) = ctx . current_type () { if ! current_type . is_composite () { ctx . report_error (vec ! [inline_fragment . pos] , format ! ("Fragment cannot condition non composite type \"{}\"" , current_type . name ()) ,) ; } } } }
    };
}

impl_193!()