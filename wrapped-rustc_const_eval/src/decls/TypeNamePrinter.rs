macro_rules! TypeNamePrinter {
    () => {
        struct TypeNamePrinter < 'tcx > { tcx : TyCtxt < 'tcx > , path : String , }
    };
}

TypeNamePrinter!();