macro_rules! deps {
    () => {
        Attrs!();
        EnumVariants!();
        EnumVariantLoc!();
        DefDatabase!();
        InactiveEnumVariantCode!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        # [salsa :: tracked] impl EnumVariants { # [salsa :: tracked (returns (ref))] pub (crate) fn of (db : & dyn DefDatabase , e : EnumId ,) -> (EnumVariants , Option < ThinVec < InactiveEnumVariantCode > >) { let loc = e . lookup (db) ; let source = loc . source (db) ; let ast_id_map = db . ast_id_map (source . file_id) ; let span_map = db . span_map (source . file_id) ; let mut diagnostics = ThinVec :: new () ; let cfg_options = loc . container . krate . cfg_options (db) ; let mut index = 0 ; let Some (variants) = source . value . variant_list () else { return (EnumVariants { variants : Box :: default () } , None) ; } ; let variants = variants . variants () . filter_map (| variant | { let ast_id = ast_id_map . ast_id (& variant) ; match Attrs :: is_cfg_enabled_for (db , & variant , span_map . as_ref () , cfg_options) { Ok (()) => { let enum_variant = EnumVariantLoc { id : source . with_value (ast_id) , parent : e , index } . intern (db) ; index += 1 ; let name = as_name_opt (variant . name ()) ; let shape = adt_shape (variant . kind ()) ; Some ((enum_variant , name , shape)) } Err (cfg) => { diagnostics . push (InactiveEnumVariantCode { ast_id , cfg , opts : cfg_options . clone () , }) ; None } } }) . collect () ; (EnumVariants { variants } , diagnostics . is_empty () . not () . then_some (diagnostics)) } }
    };
}

impl_124!();