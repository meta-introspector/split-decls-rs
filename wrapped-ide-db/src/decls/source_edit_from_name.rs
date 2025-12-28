macro_rules! deps {
    () => {
        TextEditBuilder!();
    };
}

macro_rules! source_edit_from_name {
    () => {
        deps!();
        fn source_edit_from_name (edit : & mut TextEditBuilder , name : & ast :: Name , new_name : & dyn Display ,) -> bool { if ast :: RecordPatField :: for_field_name (name) . is_some () && let Some (ident_pat) = name . syntax () . parent () . and_then (ast :: IdentPat :: cast) { cov_mark :: hit ! (rename_record_pat_field_name_split) ; edit . insert (ident_pat . syntax () . text_range () . start () , format ! ("{new_name}: ")) ; return true ; } false }
    };
}

source_edit_from_name!()