// Generated macro for impl_714 (impl)
macro_rules! Depcrate_output_render_permissionsimpl_714 {
() => {
// Module: crate::output::render::permissions
// Provides: {"impl_714"}
// Dependencies: {}
impl PermissionsPlusRender for Option < f :: PermissionsPlus > { # [cfg (unix)] fn render < C : Colours + FiletypeColours > (& self , colours : & C) -> TextCell { match self { Some (p) => { let mut chars = vec ! [p . file_type . render (colours)] ; let permissions = p . permissions ; chars . extend (Some (permissions) . render (colours , p . file_type . is_regular_file ())) ; if p . xattrs { chars . push (colours . attribute () . paint ("@")) ; } TextCell { width : DisplayWidth :: from (chars . len ()) , contents : chars . into () , } } None => { let chars : Vec < _ > = iter :: repeat (colours . dash () . paint ("-")) . take (10) . collect () ; TextCell { width : DisplayWidth :: from (chars . len ()) , contents : chars . into () , } } } } # [cfg (windows)] fn render < C : Colours + FiletypeColours > (& self , colours : & C) -> TextCell { match self { Some (p) => { let mut chars = vec ! [p . attributes . render_type (colours)] ; chars . extend (p . attributes . render (colours)) ; TextCell { width : DisplayWidth :: from (chars . len ()) , contents : chars . into () , } } None => TextCell { width : DisplayWidth :: from (0) , contents : vec ! [] . into () , } , } } }
};
}
