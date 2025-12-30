// Generated macro for HirDisplayWrapper (struct)
macro_rules! Depcrate_displayHirDisplayWrapper {
() => {
// Module: crate::display
// Provides: {"HirDisplayWrapper"}
// Dependencies: {}
pub struct HirDisplayWrapper < 'a , 'db , T > { db : & 'db dyn HirDatabase , t : & 'a T , max_size : Option < usize > , limited_size : Option < usize > , omit_verbose_types : bool , closure_style : ClosureStyle , display_kind : DisplayKind , display_target : DisplayTarget , show_container_bounds : bool , display_lifetimes : DisplayLifetime , }
};
}
