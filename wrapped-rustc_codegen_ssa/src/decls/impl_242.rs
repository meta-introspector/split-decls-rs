macro_rules! deps {
    () => {
        Subdiagnostic!();
        SharedEmitter!();
        SharedEmitterMessage!();
        Diagnostic!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl Emitter for SharedEmitter { fn emit_diagnostic (& mut self , mut diag : rustc_errors :: DiagInner , _registry : & rustc_errors :: registry :: Registry ,) { assert_eq ! (diag . span , MultiSpan :: new ()) ; assert_eq ! (diag . suggestions , Suggestions :: Enabled (vec ! [])) ; assert_eq ! (diag . sort_span , rustc_span :: DUMMY_SP) ; assert_eq ! (diag . is_lint , None) ; let args = mem :: replace (& mut diag . args , DiagArgMap :: default ()) ; drop (self . sender . send (SharedEmitterMessage :: Diagnostic (Diagnostic { level : diag . level () , messages : diag . messages , code : diag . code , children : diag . children . into_iter () . map (| child | Subdiagnostic { level : child . level , messages : child . messages }) . collect () , args , })) ,) ; } fn source_map (& self) -> Option < & SourceMap > { None } fn translator (& self) -> & Translator { panic ! ("shared emitter attempted to translate a diagnostic") ; } }
    };
}

impl_242!();