// Generated macro for impl_140 (impl)
macro_rules! Depcrateimpl_140 {
() => {
// Module: crate
// Provides: {"impl_140"}
// Dependencies: {}
impl TzifTransitionsOwned { fn quote (& self) -> proc_macro2 :: TokenStream { let TzifTransitionsOwned { ref timestamps , ref civil_starts , ref civil_ends , ref infos , } = * self ; let civil_starts : Vec < _ > = civil_starts . iter () . map (TzifDateTime :: quote) . collect () ; let civil_ends : Vec < _ > = civil_ends . iter () . map (TzifDateTime :: quote) . collect () ; let infos : Vec < _ > = infos . iter () . map (TzifTransitionInfo :: quote) . collect () ; quote ! { jiff :: shared :: TzifTransitions { timestamps : & [# (# timestamps) ,*] , civil_starts : & [# (# civil_starts) ,*] , civil_ends : & [# (# civil_ends) ,*] , infos : & [# (# infos) ,*] , } } } }
};
}
