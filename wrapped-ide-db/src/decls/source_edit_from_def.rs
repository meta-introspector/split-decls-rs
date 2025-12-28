macro_rules! deps {
    () => {
        RootDatabase!();
        ChangeAnnotation!();
        Result!();
        FileRange!();
        Definition!();
        TextEdit!();
        SourceChange!();
    };
}

macro_rules! source_edit_from_def {
    () => {
        deps!();
        fn source_edit_from_def (sema : & Semantics < '_ , RootDatabase > , def : Definition , new_name : & Name , source_change : & mut SourceChange ,) -> Result < (FileId , TextEdit) > { let mut edit = TextEdit :: builder () ; if let Definition :: Local (local) = def { let mut file_id = None ; let conflict_annotation = if ! sema . rename_conflicts (& local , new_name) . is_empty () { Some (source_change . insert_annotation (ChangeAnnotation { label : "This rename will change the program's meaning" . to_owned () , needs_confirmation : true , description : Some ("Some variable(s) will shadow the renamed variable \
                        or be shadowed by it if the rename is performed" . to_owned () ,) , }) ,) } else { None } ; for source in local . sources (sema . db) { let source = match source . source . clone () . original_ast_node_rooted (sema . db) { Some (source) => source , None => match source . source . syntax () . original_file_range_opt (sema . db) . map (TupleExt :: head) { Some (FileRange { file_id : file_id2 , range }) => { file_id = Some (file_id2) ; edit . replace (range , new_name . display (sema . db , file_id2 . edition (sema . db)) . to_string () ,) ; continue ; } None => { bail ! ("Can't rename local that is defined in a macro declaration") } } , } ; file_id = Some (source . file_id) ; if let Either :: Left (pat) = source . value { let name_range = pat . name () . unwrap () . syntax () . text_range () ; if let Some (pat_field) = pat . syntax () . parent () . and_then (ast :: RecordPatField :: cast) { if let Some (name_ref) = pat_field . name_ref () { if new_name . as_str () == name_ref . text () . as_str () . trim_start_matches ("r#") && pat . at_token () . is_none () { cov_mark :: hit ! (test_rename_local_put_init_shorthand_pat) ; edit . delete (name_ref . syntax () . text_range () . cover_offset (pat . syntax () . text_range () . start ()) ,) ; edit . replace (name_range , name_ref . text () . to_string ()) ; } else { edit . replace (name_range , new_name . display (sema . db , source . file_id . edition (sema . db)) . to_string () ,) ; } } else { edit . insert (pat . syntax () . text_range () . start () , format ! ("{}: " , pat_field . field_name () . unwrap ()) ,) ; edit . replace (name_range , new_name . display (sema . db , source . file_id . edition (sema . db)) . to_string () ,) ; } } else { edit . replace (name_range , new_name . display (sema . db , source . file_id . edition (sema . db)) . to_string () ,) ; } } } let mut edit = edit . finish () ; for (edit , _) in source_change . source_file_edits . values_mut () { edit . set_annotation (conflict_annotation) ; } edit . set_annotation (conflict_annotation) ; let Some (file_id) = file_id else { bail ! ("No file available to rename") } ; return Ok ((file_id . file_id (sema . db) , edit)) ; } let FileRange { file_id , range } = def . range_for_rename (sema) . ok_or_else (| | format_err ! ("No identifier available to rename")) ? ; let (range , new_name) = match def { Definition :: ExternCrateDecl (decl) if decl . alias (sema . db) . is_none () => (TextRange :: empty (range . end ()) , format ! (" as {}" , new_name . display (sema . db , file_id . edition (sema . db)) ,) ,) , _ => (range , new_name . display (sema . db , file_id . edition (sema . db)) . to_string ()) , } ; edit . replace (range , new_name) ; Ok ((file_id . file_id (sema . db) , edit . finish ())) }
    };
}

source_edit_from_def!()