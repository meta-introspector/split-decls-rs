macro_rules! deps {
    () => {
        DefDatabase!();
        VariantId!();
        Attrs!();
        LocalFieldId!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Attrs { pub const EMPTY : Self = Self (RawAttrs :: EMPTY) ; pub (crate) fn fields_attrs_query (db : & dyn DefDatabase , v : VariantId ,) -> Arc < ArenaMap < LocalFieldId , Attrs > > { let _p = tracing :: info_span ! ("fields_attrs_query") . entered () ; let mut res = ArenaMap :: default () ; let (fields , file_id , krate) = match v { VariantId :: EnumVariantId (it) => { let loc = it . lookup (db) ; let krate = loc . parent . lookup (db) . container . krate ; let source = loc . source (db) ; (source . value . field_list () , source . file_id , krate) } VariantId :: StructId (it) => { let loc = it . lookup (db) ; let krate = loc . container . krate ; let source = loc . source (db) ; (source . value . field_list () , source . file_id , krate) } VariantId :: UnionId (it) => { let loc = it . lookup (db) ; let krate = loc . container . krate ; let source = loc . source (db) ; (source . value . record_field_list () . map (ast :: FieldList :: RecordFieldList) , source . file_id , krate ,) } } ; let Some (fields) = fields else { return Arc :: new (res) ; } ; let cfg_options = krate . cfg_options (db) ; let span_map = db . span_map (file_id) ; match fields { ast :: FieldList :: RecordFieldList (fields) => { let mut idx = 0 ; for field in fields . fields () { let attrs = Attrs (RawAttrs :: new_expanded (db , & field , span_map . as_ref () , cfg_options)) ; if attrs . is_cfg_enabled (cfg_options) . is_ok () { res . insert (Idx :: from_raw (RawIdx :: from (idx)) , attrs) ; idx += 1 ; } } } ast :: FieldList :: TupleFieldList (fields) => { let mut idx = 0 ; for field in fields . fields () { let attrs = Attrs (RawAttrs :: new_expanded (db , & field , span_map . as_ref () , cfg_options)) ; if attrs . is_cfg_enabled (cfg_options) . is_ok () { res . insert (Idx :: from_raw (RawIdx :: from (idx)) , attrs) ; idx += 1 ; } } } } res . shrink_to_fit () ; Arc :: new (res) } }
    };
}

impl_14!();