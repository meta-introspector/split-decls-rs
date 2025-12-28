macro_rules! deps {
    () => {
        ExpandDatabase!();
    };
}

macro_rules! real_span_map {
    () => {
        deps!();
        pub (crate) fn real_span_map (db : & dyn ExpandDatabase , editioned_file_id : base_db :: EditionedFileId ,) -> Arc < RealSpanMap > { use syntax :: ast :: HasModuleItem ; let mut pairs = vec ! [(syntax :: TextSize :: new (0) , span :: ROOT_ERASED_FILE_AST_ID)] ; let ast_id_map = db . ast_id_map (editioned_file_id . into ()) ; let tree = db . parse (editioned_file_id) . tree () ; let item_to_entry = | item : ast :: Item | (item . syntax () . text_range () . start () , ast_id_map . ast_id (& item) . erase ()) ; pairs . extend (tree . items () . map (item_to_entry)) ; tree . items () . for_each (| item | match & item { ast :: Item :: ExternBlock (it) if ! collect_attrs (it) . map (TupleExt :: tail) . any (| it | it . is_left ()) => { if let Some (extern_item_list) = it . extern_item_list () { pairs . extend (extern_item_list . extern_items () . map (ast :: Item :: from) . map (item_to_entry) ,) ; } } ast :: Item :: Impl (it) if ! collect_attrs (it) . map (TupleExt :: tail) . any (| it | it . is_left ()) => { if let Some (assoc_item_list) = it . assoc_item_list () { pairs . extend (assoc_item_list . assoc_items () . map (ast :: Item :: from) . map (item_to_entry)) ; } } ast :: Item :: Module (it) if ! collect_attrs (it) . map (TupleExt :: tail) . any (| it | it . is_left ()) => { if let Some (item_list) = it . item_list () { pairs . extend (item_list . items () . map (item_to_entry)) ; } } ast :: Item :: Trait (it) if ! collect_attrs (it) . map (TupleExt :: tail) . any (| it | it . is_left ()) => { if let Some (assoc_item_list) = it . assoc_item_list () { pairs . extend (assoc_item_list . assoc_items () . map (ast :: Item :: from) . map (item_to_entry)) ; } } _ => () , }) ; Arc :: new (RealSpanMap :: from_file (editioned_file_id . editioned_file_id (db) , pairs . into_boxed_slice () , tree . syntax () . text_range () . end () ,)) }
    };
}

real_span_map!()