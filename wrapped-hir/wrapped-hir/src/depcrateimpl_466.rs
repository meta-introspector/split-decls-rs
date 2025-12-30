// Generated macro for impl_466 (impl)
macro_rules! Depcrateimpl_466 {
() => {
// Module: crate
// Provides: {"impl_466"}
// Dependencies: {}
impl Closure { fn as_ty (self) -> Ty { TyKind :: Closure (self . id , self . subst) . intern (Interner) } pub fn display_with_id (& self , db : & dyn HirDatabase , display_target : DisplayTarget) -> String { self . clone () . as_ty () . display (db , display_target) . with_closure_style (ClosureStyle :: ClosureWithId) . to_string () } pub fn display_with_impl (& self , db : & dyn HirDatabase , display_target : DisplayTarget) -> String { self . clone () . as_ty () . display (db , display_target) . with_closure_style (ClosureStyle :: ImplFn) . to_string () } pub fn captured_items (& self , db : & dyn HirDatabase) -> Vec < ClosureCapture > { let owner = db . lookup_intern_closure ((self . id) . into ()) . 0 ; let infer = & db . infer (owner) ; let info = infer . closure_info (& self . id) ; info . 0 . iter () . cloned () . map (| capture | ClosureCapture { owner , closure : self . id , capture }) . collect () } pub fn capture_types < 'db > (& self , db : & 'db dyn HirDatabase) -> Vec < Type < 'db > > { let owner = db . lookup_intern_closure ((self . id) . into ()) . 0 ; let infer = & db . infer (owner) ; let (captures , _) = infer . closure_info (& self . id) ; captures . iter () . map (| capture | Type { env : db . trait_environment_for_body (owner) , ty : capture . ty (& self . subst) , _pd : PhantomCovariantLifetime :: new () , }) . collect () } pub fn fn_trait (& self , db : & dyn HirDatabase) -> FnTrait { let owner = db . lookup_intern_closure ((self . id) . into ()) . 0 ; let infer = & db . infer (owner) ; let info = infer . closure_info (& self . id) ; info . 1 } }
};
}
