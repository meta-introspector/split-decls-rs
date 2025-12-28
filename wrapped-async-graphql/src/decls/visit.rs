macro_rules! deps {
    () => {
        Visitor!();
        VisitorContext!();
    };
}

macro_rules! visit {
    () => {
        deps!();
        pub (crate) fn visit < 'a , V : Visitor < 'a > > (v : & mut V , ctx : & mut VisitorContext < 'a > , doc : & 'a ExecutableDocument ,) { v . enter_document (ctx , doc) ; for (name , fragment) in & doc . fragments { ctx . with_type (ctx . registry . types . get (fragment . node . type_condition . node . on . node . as_str ()) , | ctx | visit_fragment_definition (v , ctx , name , fragment) ,) } for (name , operation) in doc . operations . iter () { visit_operation_definition (v , ctx , name , operation) ; } v . exit_document (ctx , doc) ; }
    };
}

visit!()