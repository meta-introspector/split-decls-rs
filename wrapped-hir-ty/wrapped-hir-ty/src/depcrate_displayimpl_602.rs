// Generated macro for impl_602 (impl)
macro_rules! Depcrate_displayimpl_602 {
() => {
// Module: crate::display
// Provides: {"impl_602"}
// Dependencies: {}
impl < T : HirDisplay > HirDisplayWrapper < '_ , T > { pub fn write_to < F : HirWrite > (& self , f : & mut F) -> Result < () , HirDisplayError > { self . t . hir_fmt (& mut HirFormatter { db : self . db , fmt : f , buf : String :: with_capacity (self . max_size . unwrap_or (20)) , curr_size : 0 , max_size : self . max_size , entity_limit : self . limited_size , omit_verbose_types : self . omit_verbose_types , display_kind : self . display_kind , display_target : self . display_target , closure_style : self . closure_style , show_container_bounds : self . show_container_bounds , display_lifetimes : self . display_lifetimes , bounds_formatting_ctx : Default :: default () , }) } pub fn with_closure_style (mut self , c : ClosureStyle) -> Self { self . closure_style = c ; self } pub fn with_lifetime_display (mut self , l : DisplayLifetime) -> Self { self . display_lifetimes = l ; self } }
};
}
