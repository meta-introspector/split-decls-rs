macro_rules! deps {
    () => {
        ClosureCapture!();
        Type!();
        Closure!();
        AnyClosureId!();
    };
}

macro_rules! impl_383 {
    () => {
        deps!();
        impl < 'db > Closure < 'db > { fn as_ty (& self , db : & 'db dyn HirDatabase) -> Ty < 'db > { let interner = DbInterner :: new_with (db , None , None) ; match self . id { AnyClosureId :: ClosureId (id) => Ty :: new_closure (interner , id . into () , self . subst) , AnyClosureId :: CoroutineClosureId (id) => { Ty :: new_coroutine_closure (interner , id . into () , self . subst) } } } pub fn display_with_id (& self , db : & dyn HirDatabase , display_target : DisplayTarget) -> String { self . as_ty (db) . display (db , display_target) . with_closure_style (ClosureStyle :: ClosureWithId) . to_string () } pub fn display_with_impl (& self , db : & dyn HirDatabase , display_target : DisplayTarget) -> String { self . as_ty (db) . display (db , display_target) . with_closure_style (ClosureStyle :: ImplFn) . to_string () } pub fn captured_items (& self , db : & 'db dyn HirDatabase) -> Vec < ClosureCapture < 'db > > { let AnyClosureId :: ClosureId (id) = self . id else { return Vec :: new () ; } ; let owner = db . lookup_intern_closure (id) . 0 ; let infer = db . infer (owner) ; let info = infer . closure_info (id) ; info . 0 . iter () . cloned () . map (| capture | ClosureCapture { owner , closure : id , capture }) . collect () } pub fn capture_types (& self , db : & 'db dyn HirDatabase) -> Vec < Type < 'db > > { let AnyClosureId :: ClosureId (id) = self . id else { return Vec :: new () ; } ; let owner = db . lookup_intern_closure (id) . 0 ; let infer = db . infer (owner) ; let (captures , _) = infer . closure_info (id) ; let env = db . trait_environment_for_body (owner) ; captures . iter () . map (| capture | Type { env : env . clone () , ty : capture . ty (db , self . subst) }) . collect () } pub fn fn_trait (& self , db : & dyn HirDatabase) -> FnTrait { match self . id { AnyClosureId :: ClosureId (id) => { let owner = db . lookup_intern_closure (id) . 0 ; let infer = db . infer (owner) ; let info = infer . closure_info (id) ; info . 1 } AnyClosureId :: CoroutineClosureId (_id) => { match self . subst . as_coroutine_closure () . kind () { rustc_type_ir :: ClosureKind :: Fn => FnTrait :: AsyncFn , rustc_type_ir :: ClosureKind :: FnMut => FnTrait :: AsyncFnMut , rustc_type_ir :: ClosureKind :: FnOnce => FnTrait :: AsyncFnOnce , } } } } }
    };
}

impl_383!()