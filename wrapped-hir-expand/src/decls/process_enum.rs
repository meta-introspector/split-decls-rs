macro_rules! deps {
    () => {
        ExpandDatabase!();
    };
}

macro_rules! process_enum {
    () => {
        deps!();
        fn process_enum (db : & dyn ExpandDatabase , variants : VariantList , krate : Crate , remove : & mut FxHashSet < SyntaxElement > ,) -> Option < () > { 'variant : for variant in variants . variants () { for attr in variant . attrs () { if let Some (enabled) = check_cfg (db , & attr , krate) { if enabled { debug ! ("censoring {:?}" , attr . syntax ()) ; remove . insert (attr . syntax () . clone () . into ()) ; } else { debug ! ("censoring type {:?}" , variant . syntax ()) ; remove . insert (variant . syntax () . clone () . into ()) ; remove_possible_comma (& variant , remove) ; continue 'variant ; } } if let Some (enabled) = check_cfg_attr (db , & attr , krate) { if enabled { debug ! ("Removing cfg_attr tokens {:?}" , attr) ; let meta = attr . meta () ? ; let removes_from_cfg_attr = remove_tokens_within_cfg_attr (meta) ? ; remove . extend (removes_from_cfg_attr) ; } else { debug ! ("censoring type cfg_attr {:?}" , variant . syntax ()) ; remove . insert (attr . syntax () . clone () . into ()) ; } } } if let Some (fields) = variant . field_list () { match fields { ast :: FieldList :: RecordFieldList (fields) => { process_has_attrs_with_possible_comma (db , fields . fields () , krate , remove) ? ; } ast :: FieldList :: TupleFieldList (fields) => { process_has_attrs_with_possible_comma (db , fields . fields () , krate , remove) ? ; } } } } Some (()) }
    };
}

process_enum!()