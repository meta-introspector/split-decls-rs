macro_rules! deps {
    () => {
        HasChildSource!();
        VariantId!();
        DefDatabase!();
        Attrs!();
        LocalFieldId!();
    };
}

macro_rules! impl_394 {
    () => {
        deps!();
        impl HasChildSource < LocalFieldId > for VariantId { type Value = Either < ast :: TupleField , ast :: RecordField > ; fn child_source (& self , db : & dyn DefDatabase) -> InFile < ArenaMap < LocalFieldId , Self :: Value > > { let (src , container) = match * self { VariantId :: EnumVariantId (it) => { let lookup = it . lookup (db) ; (lookup . source (db) . map (| it | it . kind ()) , lookup . parent . lookup (db) . container) } VariantId :: StructId (it) => { let lookup = it . lookup (db) ; (lookup . source (db) . map (| it | it . kind ()) , lookup . container) } VariantId :: UnionId (it) => { let lookup = it . lookup (db) ; (lookup . source (db) . map (| it | it . kind ()) , lookup . container) } } ; let span_map = db . span_map (src . file_id) ; let mut map = ArenaMap :: new () ; match & src . value { ast :: StructKind :: Tuple (fl) => { let cfg_options = container . krate . cfg_options (db) ; let mut idx = 0 ; for fd in fl . fields () { let enabled = Attrs :: is_cfg_enabled_for (db , & fd , span_map . as_ref () , cfg_options) . is_ok () ; if ! enabled { continue ; } map . insert (LocalFieldId :: from_raw (la_arena :: RawIdx :: from (idx)) , Either :: Left (fd . clone ()) ,) ; idx += 1 ; } } ast :: StructKind :: Record (fl) => { let cfg_options = container . krate . cfg_options (db) ; let mut idx = 0 ; for fd in fl . fields () { let enabled = Attrs :: is_cfg_enabled_for (db , & fd , span_map . as_ref () , cfg_options) . is_ok () ; if ! enabled { continue ; } map . insert (LocalFieldId :: from_raw (la_arena :: RawIdx :: from (idx)) , Either :: Right (fd . clone ()) ,) ; idx += 1 ; } } ast :: StructKind :: Unit => () , } InFile :: new (src . file_id , map) } }
    };
}

impl_394!()