macro_rules! deps {
    () => {
        RenameResult!();
    };
}

macro_rules! rename_self_to_param {
    () => {
        deps!();
        fn rename_self_to_param (sema : & Semantics < '_ , RootDatabase > , local : hir :: Local , self_param : hir :: SelfParam , new_name : & Name , identifier_kind : IdentifierKind , find_path_config : FindPathConfig ,) -> RenameResult < SourceChange > { if identifier_kind == IdentifierKind :: LowercaseSelf { cov_mark :: hit ! (rename_self_to_self) ; return Ok (SourceChange :: default ()) ; } let fn_def = match local . parent (sema . db) { hir :: DefWithBody :: Function (func) => func , _ => bail ! ("Cannot rename local to self outside of function") , } ; let InFile { file_id , value : self_param } = sema . source (self_param) . ok_or_else (| | format_err ! ("cannot find function source")) ? ; let def = Definition :: Local (local) ; let usages = def . usages (sema) . all () ; let edit = text_edit_from_self_param (& self_param , new_name . display (sema . db , file_id . edition (sema . db)) . to_string () ,) . ok_or_else (| | format_err ! ("No target type found")) ? ; if usages . len () > 1 && identifier_kind == IdentifierKind :: Underscore { bail ! ("Cannot rename reference to `_` as it is being referenced multiple times") ; } let mut source_change = SourceChange :: default () ; source_change . insert_source_edit (file_id . original_file (sema . db) . file_id (sema . db) , edit) ; source_change . extend (usages . iter () . map (| (file_id , references) | { (file_id . file_id (sema . db) , source_edit_from_references (sema . db , references , def , new_name , file_id . edition (sema . db) ,) ,) })) ; transform_method_call_into_assoc_fn (sema , & mut source_change , fn_def , find_path_config) ; Ok (source_change) }
    };
}

rename_self_to_param!()