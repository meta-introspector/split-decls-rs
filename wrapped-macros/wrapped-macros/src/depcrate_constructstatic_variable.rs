// Generated macro for static_variable (function)
macro_rules! Depcrate_constructstatic_variable {
() => {
// Module: crate::construct
// Provides: {"static_variable"}
// Dependencies: {}
pub (crate) fn static_variable (name : & Ident2 , data : & str , tag : & str , prefix : Option < & str > ,) -> TokenStream2 { let sym_name = mangled_symbol_name (tag , data) ; let section = linker_section (false , prefix , & sym_name) ; let section_for_macos = linker_section (true , prefix , & sym_name) ; quote ! (# [cfg_attr (target_os = "macos" , link_section = # section_for_macos)] # [cfg_attr (not (target_os = "macos") , link_section = # section)] # [export_name = # sym_name] static # name : u8 = 0 ;) }
};
}
