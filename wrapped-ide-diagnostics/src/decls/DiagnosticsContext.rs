macro_rules! deps {
    () => {
        DiagnosticsConfig!();
    };
}

macro_rules! DiagnosticsContext {
    () => {
        deps!();
        struct DiagnosticsContext < 'a > { config : & 'a DiagnosticsConfig , sema : Semantics < 'a , RootDatabase > , resolve : & 'a AssistResolveStrategy , edition : Edition , display_target : DisplayTarget , is_nightly : bool , }
    };
}

DiagnosticsContext!();