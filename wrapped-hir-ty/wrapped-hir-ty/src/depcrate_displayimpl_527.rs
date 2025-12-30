// Generated macro for impl_527 (impl)
macro_rules! Depcrate_displayimpl_527 {
() => {
// Module: crate::display
// Provides: {"impl_527"}
// Dependencies: {}
impl < 'db , T : HirDisplay < 'db > > HirDisplayWrapper < '_ , 'db , T > { pub fn write_to < F : HirWrite > (& self , f : & mut F) -> Result < () , HirDisplayError > { let krate = self . display_target . krate ; let block = match self . display_kind { DisplayKind :: SourceCode { target_module_id , .. } => target_module_id . containing_block () , DisplayKind :: Diagnostics | DisplayKind :: Test => None , } ; let interner = DbInterner :: new_with (self . db , Some (krate) , block) ; self . t . hir_fmt (& mut HirFormatter { db : self . db , interner , fmt : f , buf : String :: with_capacity (self . max_size . unwrap_or (20)) , curr_size : 0 , max_size : self . max_size , entity_limit : self . limited_size , omit_verbose_types : self . omit_verbose_types , display_kind : self . display_kind , display_target : self . display_target , closure_style : self . closure_style , show_container_bounds : self . show_container_bounds , display_lifetimes : self . display_lifetimes , bounds_formatting_ctx : Default :: default () , }) } pub fn with_closure_style (mut self , c : ClosureStyle) -> Self { self . closure_style = c ; self } pub fn with_lifetime_display (mut self , l : DisplayLifetime) -> Self { self . display_lifetimes = l ; self } }
};
}
