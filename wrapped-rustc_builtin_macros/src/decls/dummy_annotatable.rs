macro_rules! dummy_annotatable {
    () => {
        fn dummy_annotatable () -> Annotatable { Annotatable :: GenericParam (ast :: GenericParam { id : ast :: DUMMY_NODE_ID , ident : Ident :: dummy () , attrs : Default :: default () , bounds : Default :: default () , is_placeholder : false , kind : GenericParamKind :: Lifetime , colon_span : None , }) }
    };
}

dummy_annotatable!()