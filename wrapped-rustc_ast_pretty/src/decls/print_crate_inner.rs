macro_rules! deps {
    () => {
        State!();
        AnnNode!();
    };
}

macro_rules! print_crate_inner {
    () => {
        deps!();
        fn print_crate_inner < 'a > (s : & mut State < 'a > , krate : & ast :: Crate , is_expanded : bool , edition : Edition , g : & AttrIdGenerator ,) { s . maybe_print_shebang () ; if is_expanded && ! krate . attrs . iter () . any (| attr | attr . has_name (sym :: no_core)) { let fake_attr = attr :: mk_attr_nested_word (g , ast :: AttrStyle :: Inner , Safety :: Default , sym :: feature , sym :: prelude_import , DUMMY_SP ,) ; s . print_attribute (& fake_attr) ; if edition . is_rust_2015 () { let fake_attr = attr :: mk_attr_word (g , ast :: AttrStyle :: Inner , Safety :: Default , sym :: no_std , DUMMY_SP ,) ; s . print_attribute (& fake_attr) ; } } s . print_inner_attributes (& krate . attrs) ; for item in & krate . items { s . print_item (item) ; } s . print_remaining_comments () ; s . ann . post (s , AnnNode :: Crate (krate)) ; }
    };
}

print_crate_inner!()