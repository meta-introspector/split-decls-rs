macro_rules! deps {
    () => {
        FindAllRefsConfig!();
        Annotation!();
        AnnotationKind!();
        AnnotationConfig!();
        GotoImplementationConfig!();
    };
}

macro_rules! resolve_annotation {
    () => {
        deps!();
        pub (crate) fn resolve_annotation (db : & RootDatabase , config : & AnnotationConfig < '_ > , mut annotation : Annotation ,) -> Annotation { match annotation . kind { AnnotationKind :: HasImpls { pos , ref mut data } => { let goto_implementation_config = GotoImplementationConfig { filter_adjacent_derive_implementations : config . filter_adjacent_derive_implementations , } ; * data = goto_implementation (db , & goto_implementation_config , pos) . map (| range | range . info) ; } AnnotationKind :: HasReferences { pos , ref mut data } => { * data = find_all_refs (& Semantics :: new (db) , pos , & FindAllRefsConfig { search_scope : None , minicore : config . minicore } ,) . map (| result | { result . into_iter () . flat_map (| res | res . references) . flat_map (| (file_id , access) | { access . into_iter () . map (move | (range , _) | FileRange { file_id , range }) }) . collect () }) ; } _ => { } } ; annotation }
    };
}

resolve_annotation!();