macro_rules! deps {
    () => {
        NavigationTarget!();
    };
}

macro_rules! try_filter_trait_item_definition {
    () => {
        deps!();
        # [doc = " finds the trait definition of an impl'd item, except function"] # [doc = " e.g."] # [doc = " ```rust"] # [doc = " trait A { type a; }"] # [doc = " struct S;"] # [doc = " impl A for S { type a = i32; } // <-- on this associate type, will get the location of a in the trait"] # [doc = " ```"] fn try_filter_trait_item_definition (sema : & Semantics < '_ , RootDatabase > , def : & Definition ,) -> Option < Vec < NavigationTarget > > { let db = sema . db ; let assoc = def . as_assoc_item (db) ? ; match assoc { AssocItem :: Function (..) => None , AssocItem :: Const (..) | AssocItem :: TypeAlias (..) => { let trait_ = assoc . implemented_trait (db) ? ; let name = def . name (db) ? ; let discriminant_value = discriminant (& assoc) ; trait_ . items (db) . iter () . filter (| itm | discriminant (* itm) == discriminant_value) . find_map (| itm | (itm . name (db) ? == name) . then (| | itm . try_to_nav (sema)) . flatten ()) . map (| it | it . collect ()) } } }
    };
}

try_filter_trait_item_definition!()