macro_rules! deps {
    () => {
        Definition!();
    };
}

macro_rules! as_trait_assoc_def {
    () => {
        deps!();
        # [doc = " If this is an trait (impl) assoc item, returns the assoc item of the corresponding trait definition."] pub (crate) fn as_trait_assoc_def (db : & dyn HirDatabase , def : Definition) -> Option < Definition > { let assoc = def . as_assoc_item (db) ? ; let trait_ = match assoc . container (db) { hir :: AssocItemContainer :: Trait (_) => return Some (def) , hir :: AssocItemContainer :: Impl (i) => i . trait_ (db) , } ? ; assoc_item_of_trait (db , assoc , trait_) }
    };
}

as_trait_assoc_def!()