macro_rules! deps {
    () => {
        ModuleId!();
        DefDatabase!();
        ExpressionStoreSourceMap!();
        Visibility!();
        ExpressionStore!();
        FieldData!();
    };
}

macro_rules! lower_field_list {
    () => {
        deps!();
        fn lower_field_list (db : & dyn DefDatabase , module : ModuleId , fields : InFile < Option < ast :: FieldList > > , override_visibility : Option < Option < ast :: Visibility > > ,) -> Option < (Arena < FieldData > , ExpressionStore , ExpressionStoreSourceMap) > { let file_id = fields . file_id ; match fields . value ? { ast :: FieldList :: RecordFieldList (fields) => lower_fields (db , module , InFile :: new (file_id , fields . fields () . map (| field | (field . ty () , field))) , | _ , field | as_name_opt (field . name ()) , override_visibility ,) , ast :: FieldList :: TupleFieldList (fields) => lower_fields (db , module , InFile :: new (file_id , fields . fields () . map (| field | (field . ty () , field))) , | idx , _ | Name :: new_tuple_field (idx) , override_visibility ,) , } }
    };
}

lower_field_list!();