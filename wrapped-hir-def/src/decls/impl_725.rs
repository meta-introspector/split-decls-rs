macro_rules! deps {
    () => {
        VariantId!();
        AdtId!();
        DefDatabase!();
        ExpressionStoreSourceMap!();
        VariantFields!();
    };
}

macro_rules! impl_725 {
    () => {
        deps!();
        impl VariantId { pub fn fields (self , db : & dyn DefDatabase) -> & VariantFields { VariantFields :: firewall (db , self) } pub fn fields_with_source_map (self , db : & dyn DefDatabase ,) -> (Arc < VariantFields > , Arc < ExpressionStoreSourceMap >) { VariantFields :: query (db , self) } pub fn file_id (self , db : & dyn DefDatabase) -> HirFileId { match self { VariantId :: EnumVariantId (it) => it . lookup (db) . id . file_id , VariantId :: StructId (it) => it . lookup (db) . id . file_id , VariantId :: UnionId (it) => it . lookup (db) . id . file_id , } } pub fn adt_id (self , db : & dyn DefDatabase) -> AdtId { match self { VariantId :: EnumVariantId (it) => it . lookup (db) . parent . into () , VariantId :: StructId (it) => it . into () , VariantId :: UnionId (it) => it . into () , } } }
    };
}

impl_725!()