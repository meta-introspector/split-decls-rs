macro_rules! deps {
    () => {
        VisitorContext!();
        Visitor!();
    };
}

macro_rules! visit_selection_set {
    () => {
        deps!();
        fn visit_selection_set < 'a , V : Visitor < 'a > > (v : & mut V , ctx : & mut VisitorContext < 'a > , selection_set : & 'a Positioned < SelectionSet > ,) { if ! selection_set . node . items . is_empty () { v . enter_selection_set (ctx , selection_set) ; for selection in & selection_set . node . items { visit_selection (v , ctx , selection) ; } v . exit_selection_set (ctx , selection_set) ; } }
    };
}

visit_selection_set!()