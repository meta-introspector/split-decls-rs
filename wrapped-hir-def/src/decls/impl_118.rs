macro_rules! deps {
    () => {
        DefDatabase!();
        VariantId!();
        ExpressionStoreSourceMap!();
        ExpressionStore!();
        VariantFields!();
        FieldsShape!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        # [salsa :: tracked] impl VariantFields { # [salsa :: tracked (returns (clone))] pub (crate) fn query (db : & dyn DefDatabase , id : VariantId ,) -> (Arc < Self > , Arc < ExpressionStoreSourceMap >) { let (shape , result) = match id { VariantId :: EnumVariantId (id) => { let loc = id . lookup (db) ; let parent = loc . parent . lookup (db) ; let source = loc . source (db) ; let shape = adt_shape (source . value . kind ()) ; let enum_vis = Some (source . value . parent_enum () . visibility ()) ; let fields = lower_field_list (db , parent . container , source . map (| src | src . field_list ()) , enum_vis ,) ; (shape , fields) } VariantId :: StructId (id) => { let loc = id . lookup (db) ; let source = loc . source (db) ; let shape = adt_shape (source . value . kind ()) ; let fields = lower_field_list (db , loc . container , source . map (| src | src . field_list ()) , None) ; (shape , fields) } VariantId :: UnionId (id) => { let loc = id . lookup (db) ; let source = loc . source (db) ; let fields = lower_field_list (db , loc . container , source . map (| src | src . record_field_list () . map (ast :: FieldList :: RecordFieldList)) , None ,) ; (FieldsShape :: Record , fields) } } ; match result { Some ((fields , store , source_map)) => (Arc :: new (VariantFields { fields , store : Arc :: new (store) , shape }) , Arc :: new (source_map) ,) , None => { let (store , source_map) = ExpressionStore :: empty_singleton () ; (Arc :: new (VariantFields { fields : Arena :: default () , store , shape }) , source_map) } } } # [salsa :: tracked (returns (deref))] pub (crate) fn firewall (db : & dyn DefDatabase , id : VariantId) -> Arc < Self > { Self :: query (db , id) . 0 } }
    };
}

impl_118!()