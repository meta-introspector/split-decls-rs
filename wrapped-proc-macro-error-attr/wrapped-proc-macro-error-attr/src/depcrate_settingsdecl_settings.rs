// Generated macro for decl_settings (macro)
macro_rules! Depcrate_settingsdecl_settings {
() => {
// Module: crate::settings
// Provides: {"decl_settings"}
// Dependencies: {}
macro_rules ! decl_settings { ($ ($ val : expr => $ variant : ident) ,+ $ (,) *) => { # [derive (PartialEq , Clone , Copy)] pub (crate) enum Setting { $ ($ variant) ,* } fn ident_to_setting (ident : Ident) -> Result < Setting > { match &* ident . to_string () { $ ($ val => Ok (Setting ::$ variant) ,) * _ => { let possible_vals = [$ ($ val) ,*] . iter () . map (| v | format ! ("`{}`" , v)) . collect ::< Vec < _ >> () . join (", ") ; Err (Error :: new (ident . span () , format ! ("unknown setting `{}`, expected one of {}" , ident , possible_vals))) } } } } ; }
};
}
