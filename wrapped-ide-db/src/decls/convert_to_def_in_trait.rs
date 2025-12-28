macro_rules! deps {
    () => {
        Definition!();
    };
}

macro_rules! convert_to_def_in_trait {
    () => {
        deps!();
        # [doc = " Converts associated trait impl items to their trait definition counterpart"] pub (crate) fn convert_to_def_in_trait (db : & dyn HirDatabase , def : Definition) -> Definition { (| | { let assoc = def . as_assoc_item (db) ? ; let trait_ = assoc . implemented_trait (db) ? ; assoc_item_of_trait (db , assoc , trait_) }) () . unwrap_or (def) }
    };
}

convert_to_def_in_trait!();