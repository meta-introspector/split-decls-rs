macro_rules! path_generics_source_to_ast {
    () => {
        fn path_generics_source_to_ast (path : & ast :: Path , generics_source : PathGenericsSource ,) -> Option < Either < ast :: GenericArgList , ast :: NameRef > > { Some (match generics_source { PathGenericsSource :: Segment (segment) => { let segment = hir_segment_to_ast_segment (path , segment) ? ; segment . generic_arg_list () . map (Either :: Left) . or_else (| | segment . name_ref () . map (Either :: Right)) ? } PathGenericsSource :: AssocType { segment , assoc_type } => { let segment = hir_segment_to_ast_segment (path , segment) ? ; let segment_args = segment . generic_arg_list () ? ; let assoc = hir_assoc_type_binding_to_ast (& segment_args , assoc_type) ? ; assoc . generic_arg_list () . map (Either :: Left) . or_else (| | assoc . name_ref () . map (Either :: Right)) ? } }) }
    };
}

path_generics_source_to_ast!()