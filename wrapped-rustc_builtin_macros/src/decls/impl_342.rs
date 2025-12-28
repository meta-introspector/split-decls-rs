macro_rules! deps {
    () => {
        EntryPointCleaner!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < 'a > MutVisitor for EntryPointCleaner < 'a > { fn visit_item (& mut self , item : & mut ast :: Item) { self . depth += 1 ; ast :: mut_visit :: walk_item (self , item) ; self . depth -= 1 ; match entry_point_type (& item , self . depth == 0) { EntryPointType :: MainNamed | EntryPointType :: RustcMainAttr => { let allow_dead_code = attr :: mk_attr_nested_word (& self . sess . psess . attr_id_generator , ast :: AttrStyle :: Outer , ast :: Safety :: Default , sym :: allow , sym :: dead_code , self . def_site ,) ; item . attrs . retain (| attr | ! attr . has_name (sym :: rustc_main)) ; item . attrs . push (allow_dead_code) ; } EntryPointType :: None | EntryPointType :: OtherMain => { } } ; } }
    };
}

impl_342!();