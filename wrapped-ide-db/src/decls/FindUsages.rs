macro_rules! deps {
    () => {
        Definition!();
        SearchScope!();
        RootDatabase!();
    };
}

macro_rules! FindUsages {
    () => {
        deps!();
        # [derive (Clone)] pub struct FindUsages < 'a > { def : Definition , rename : Option < & 'a Rename > , sema : & 'a Semantics < 'a , RootDatabase > , scope : Option < & 'a SearchScope > , # [doc = " The container of our definition should it be an assoc item"] assoc_item_container : Option < hir :: AssocItemContainer > , # [doc = " whether to search for the `Self` type of the definition"] include_self_kw_refs : Option < hir :: Type < 'a > > , # [doc = " whether to search for the `self` module"] search_self_mod : bool , }
    };
}

FindUsages!()