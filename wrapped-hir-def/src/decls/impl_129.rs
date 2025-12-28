macro_rules! deps {
    () => {
        GeneralConstId!();
        GenericDefId!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl GeneralConstId { pub fn generic_def (self , _db : & dyn DefDatabase) -> Option < GenericDefId > { match self { GeneralConstId :: ConstId (it) => Some (it . into ()) , GeneralConstId :: StaticId (it) => Some (it . into ()) , } } pub fn name (self , db : & dyn DefDatabase) -> String { match self { GeneralConstId :: StaticId (it) => { db . static_signature (it) . name . display (db , Edition :: CURRENT) . to_string () } GeneralConstId :: ConstId (const_id) => { db . const_signature (const_id) . name . as_ref () . map_or_else (| | "_" . to_owned () , | name | name . display (db , Edition :: CURRENT) . to_string () ,) } } } }
    };
}

impl_129!()