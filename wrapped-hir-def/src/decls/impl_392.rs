macro_rules! deps {
    () => {
        DefDatabase!();
        GenericDefId!();
        HasChildSource!();
        Trait!();
    };
}

macro_rules! impl_392 {
    () => {
        deps!();
        impl HasChildSource < LocalTypeOrConstParamId > for GenericDefId { type Value = Either < ast :: TypeOrConstParam , ast :: Trait > ; fn child_source (& self , db : & dyn DefDatabase ,) -> InFile < ArenaMap < LocalTypeOrConstParamId , Self :: Value > > { let generic_params = db . generic_params (* self) ; let mut idx_iter = generic_params . iter_type_or_consts () . map (| (idx , _) | idx) ; let (file_id , generic_params_list) = self . file_id_and_params_of (db) ; let mut params = ArenaMap :: default () ; match * self { GenericDefId :: TraitId (id) => { let trait_ref = id . lookup (db) . source (db) . value ; let idx = idx_iter . next () . unwrap () ; params . insert (idx , Either :: Right (trait_ref)) ; } _ => { } } if let Some (generic_params_list) = generic_params_list { for (idx , ast_param) in idx_iter . zip (generic_params_list . type_or_const_params ()) { params . insert (idx , Either :: Left (ast_param)) ; } } InFile :: new (file_id , params) } }
    };
}

impl_392!()