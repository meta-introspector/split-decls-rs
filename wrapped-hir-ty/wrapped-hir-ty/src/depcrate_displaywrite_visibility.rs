// Generated macro for write_visibility (function)
macro_rules! Depcrate_displaywrite_visibility {
() => {
// Module: crate::display
// Provides: {"write_visibility"}
// Dependencies: {}
pub fn write_visibility < 'db > (module_id : ModuleId , vis : Visibility , f : & mut HirFormatter < '_ , 'db > ,) -> Result < () , HirDisplayError > { match vis { Visibility :: Public => write ! (f , "pub ") , Visibility :: PubCrate (_) => write ! (f , "pub(crate) ") , Visibility :: Module (vis_id , _) => { let def_map = module_id . def_map (f . db) ; let root_module_id = def_map . module_id (DefMap :: ROOT) ; if vis_id == module_id { Ok (()) } else if root_module_id == vis_id && ! root_module_id . is_within_block () { write ! (f , "pub(crate) ") } else if module_id . containing_module (f . db) == Some (vis_id) && ! vis_id . is_block_module () { write ! (f , "pub(super) ") } else { write ! (f , "pub(in ...) ") } } } }
};
}
