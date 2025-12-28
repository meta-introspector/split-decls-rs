macro_rules! mod_path_to_ast {
    () => {
        # [doc = " Converts the mod path struct into its ast representation."] pub fn mod_path_to_ast (path : & hir :: ModPath , edition : Edition) -> ast :: Path { let _p = tracing :: info_span ! ("mod_path_to_ast") . entered () ; let mut segments = Vec :: new () ; let mut is_abs = false ; match path . kind { hir :: PathKind :: Plain => { } hir :: PathKind :: SELF => segments . push (make :: path_segment_self ()) , hir :: PathKind :: Super (n) => segments . extend ((0 .. n) . map (| _ | make :: path_segment_super ())) , hir :: PathKind :: DollarCrate (_) | hir :: PathKind :: Crate => { segments . push (make :: path_segment_crate ()) } hir :: PathKind :: Abs => is_abs = true , } segments . extend (path . segments () . iter () . map (| segment | { make :: path_segment (make :: name_ref (& segment . display_no_db (edition) . to_smolstr ())) })) ; make :: path_from_segments (segments , is_abs) }
    };
}

mod_path_to_ast!();