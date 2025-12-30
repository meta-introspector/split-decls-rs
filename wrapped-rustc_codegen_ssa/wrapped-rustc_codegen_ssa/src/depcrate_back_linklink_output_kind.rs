// Generated macro for link_output_kind (function)
macro_rules! Depcrate_back_linklink_output_kind {
() => {
// Module: crate::back::link
// Provides: {"link_output_kind"}
// Dependencies: {}
fn link_output_kind (sess : & Session , crate_type : CrateType) -> LinkOutputKind { let kind = match (crate_type , sess . crt_static (Some (crate_type)) , sess . relocation_model ()) { (CrateType :: Executable , _ , _) if sess . is_wasi_reactor () => LinkOutputKind :: WasiReactorExe , (CrateType :: Executable , false , RelocModel :: Pic | RelocModel :: Pie) => { LinkOutputKind :: DynamicPicExe } (CrateType :: Executable , false , _) => LinkOutputKind :: DynamicNoPicExe , (CrateType :: Executable , true , RelocModel :: Pic | RelocModel :: Pie) => { LinkOutputKind :: StaticPicExe } (CrateType :: Executable , true , _) => LinkOutputKind :: StaticNoPicExe , (_ , true , _) => LinkOutputKind :: StaticDylib , (_ , false , _) => LinkOutputKind :: DynamicDylib , } ; let opts = & sess . target ; let pic_exe_supported = opts . position_independent_executables ; let static_pic_exe_supported = opts . static_position_independent_executables ; let static_dylib_supported = opts . crt_static_allows_dylibs ; match kind { LinkOutputKind :: DynamicPicExe if ! pic_exe_supported => LinkOutputKind :: DynamicNoPicExe , LinkOutputKind :: StaticPicExe if ! static_pic_exe_supported => LinkOutputKind :: StaticNoPicExe , LinkOutputKind :: StaticDylib if ! static_dylib_supported => LinkOutputKind :: DynamicDylib , _ => kind , } }
};
}
