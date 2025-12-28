macro_rules! PrintExtra {
    () => {
        pub enum PrintExtra < 'tcx > { AfterParsing { krate : & 'tcx ast :: Crate } , NeedsAstMap { tcx : TyCtxt < 'tcx > } , }
    };
}

PrintExtra!()