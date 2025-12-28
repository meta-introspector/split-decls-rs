macro_rules! deps {
    () => {
        VisitorContext!();
        Visitor!();
    };
}

macro_rules! visit_inline_fragment {
    () => {
        deps!();
        fn visit_inline_fragment < 'a , V : Visitor < 'a > > (v : & mut V , ctx : & mut VisitorContext < 'a > , inline_fragment : & 'a Positioned < InlineFragment > ,) { v . enter_inline_fragment (ctx , inline_fragment) ; visit_directives (v , ctx , & inline_fragment . node . directives) ; visit_selection_set (v , ctx , & inline_fragment . node . selection_set) ; v . exit_inline_fragment (ctx , inline_fragment) ; }
    };
}

visit_inline_fragment!();