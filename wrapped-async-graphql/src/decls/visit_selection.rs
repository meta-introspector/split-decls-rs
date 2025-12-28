macro_rules! deps {
    () => {
        Visitor!();
        Object!();
        Subscription!();
        VisitorContext!();
        MetaType!();
        Field!();
    };
}

macro_rules! visit_selection {
    () => {
        deps!();
        fn visit_selection < 'a , V : Visitor < 'a > > (v : & mut V , ctx : & mut VisitorContext < 'a > , selection : & 'a Positioned < Selection > ,) { v . enter_selection (ctx , selection) ; match & selection . node { Selection :: Field (field) => { if field . node . name . node != "__typename" { ctx . with_type (ctx . current_type () . and_then (| ty | ty . field_by_name (& field . node . name . node)) . and_then (| schema_field | { ctx . registry . concrete_type_by_name (& schema_field . ty) }) , | ctx | { visit_field (v , ctx , field) ; } ,) ; } else if ctx . current_type () . map (| ty | match ty { MetaType :: Object { is_subscription , .. } => * is_subscription , _ => false , }) == Some (true) { ctx . report_error (vec ! [field . pos] , "Unknown field \"__typename\" on type \"Subscription\"." ,) ; } } Selection :: FragmentSpread (fragment_spread) => { visit_fragment_spread (v , ctx , fragment_spread) } Selection :: InlineFragment (inline_fragment) => { if let Some (TypeCondition { on : name }) = & inline_fragment . node . type_condition . as_ref () . map (| c | & c . node) { ctx . with_type (ctx . registry . types . get (name . node . as_str ()) , | ctx | { visit_inline_fragment (v , ctx , inline_fragment) }) ; } else { visit_inline_fragment (v , ctx , inline_fragment) } } } v . exit_selection (ctx , selection) ; }
    };
}

visit_selection!();