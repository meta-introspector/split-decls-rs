// Generated macro for define_panel_descriptor (macro)
macro_rules! Depcrate_series_surfacedefine_panel_descriptor {
() => {
// Module: crate::series::surface
// Provides: {"define_panel_descriptor"}
// Dependencies: {}
macro_rules ! define_panel_descriptor { ($ name : ident , $ var1 : ident , $ var2 : ident , $ out : ident , ($ first : ident , $ second : ident) -> $ result : ident = $ output : expr) => { # [allow (clippy :: upper_case_acronyms)] pub struct $ name ; impl < X , Y , Z > Direction < X , Y , Z > for $ name { type Input1Type = $ var1 ; type Input2Type = $ var2 ; type OutputType = $ out ; fn make_coord (($ first , $ second) : (Self :: Input1Type , Self :: Input2Type) , $ result : Self :: OutputType ,) -> (X , Y , Z) { $ output } } } ; }
};
}
