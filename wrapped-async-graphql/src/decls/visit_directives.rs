macro_rules! deps {
    () => {
        MetaTypeName!();
        Visitor!();
        VisitorContext!();
        Directive!();
    };
}

macro_rules! visit_directives {
    () => {
        deps!();
        fn visit_directives < 'a , V : Visitor < 'a > > (v : & mut V , ctx : & mut VisitorContext < 'a > , directives : & 'a [Positioned < Directive >] ,) { for d in directives { v . enter_directive (ctx , d) ; let schema_directive = ctx . registry . directives . get (d . node . name . node . as_str ()) ; for (name , value) in & d . node . arguments { v . enter_argument (ctx , name , value) ; let expected_ty = schema_directive . and_then (| schema_directive | schema_directive . args . get (& * name . node)) . map (| input_ty | MetaTypeName :: create (& input_ty . ty)) ; ctx . with_input_type (expected_ty , | ctx | { visit_input_value (v , ctx , d . pos , expected_ty , & value . node) }) ; v . exit_argument (ctx , name , value) ; } v . exit_directive (ctx , d) ; } }
    };
}

visit_directives!()