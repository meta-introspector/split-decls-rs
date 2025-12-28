macro_rules! deps {
    () => {
        Trait!();
        AssocItem!();
        Crate!();
        Module!();
        Function!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl Trait { pub fn lang (db : & dyn HirDatabase , krate : Crate , name : & Name) -> Option < Trait > { LangItem :: from_name (name) ? . resolve_trait (db , krate . into ()) . map (Into :: into) } pub fn module (self , db : & dyn HirDatabase) -> Module { Module { id : self . id . lookup (db) . container } } pub fn name (self , db : & dyn HirDatabase) -> Name { db . trait_signature (self . id) . name . clone () } pub fn direct_supertraits (self , db : & dyn HirDatabase) -> Vec < Trait > { let traits = direct_super_traits (db , self . into ()) ; traits . iter () . map (| tr | Trait :: from (* tr)) . collect () } pub fn all_supertraits (self , db : & dyn HirDatabase) -> Vec < Trait > { let traits = all_super_traits (db , self . into ()) ; traits . iter () . map (| tr | Trait :: from (* tr)) . collect () } pub fn function (self , db : & dyn HirDatabase , name : impl PartialEq < Name >) -> Option < Function > { self . id . trait_items (db) . items . iter () . find (| (n , _) | name == * n) . and_then (| & (_ , it) | match it { AssocItemId :: FunctionId (id) => Some (Function { id }) , _ => None , }) } pub fn items (self , db : & dyn HirDatabase) -> Vec < AssocItem > { self . id . trait_items (db) . items . iter () . map (| (_name , it) | (* it) . into ()) . collect () } pub fn items_with_supertraits (self , db : & dyn HirDatabase) -> Vec < AssocItem > { self . all_supertraits (db) . into_iter () . flat_map (| tr | tr . items (db)) . collect () } pub fn is_auto (self , db : & dyn HirDatabase) -> bool { db . trait_signature (self . id) . flags . contains (TraitFlags :: AUTO) } pub fn is_unsafe (& self , db : & dyn HirDatabase) -> bool { db . trait_signature (self . id) . flags . contains (TraitFlags :: UNSAFE) } pub fn type_or_const_param_count (& self , db : & dyn HirDatabase , count_required_only : bool ,) -> usize { db . generic_params (self . id . into ()) . iter_type_or_consts () . filter (| (_ , ty) | ! matches ! (ty , TypeOrConstParamData :: TypeParamData (ty) if ty . provenance != TypeParamProvenance :: TypeParamList)) . filter (| (_ , ty) | ! count_required_only || ! ty . has_default ()) . count () } pub fn dyn_compatibility (& self , db : & dyn HirDatabase) -> Option < DynCompatibilityViolation > { hir_ty :: dyn_compatibility :: dyn_compatibility (db , self . id) } pub fn dyn_compatibility_all_violations (& self , db : & dyn HirDatabase ,) -> Option < Vec < DynCompatibilityViolation > > { let mut violations = vec ! [] ; _ = hir_ty :: dyn_compatibility :: dyn_compatibility_with_callback (db , self . id , & mut | violation | { violations . push (violation) ; ControlFlow :: Continue (()) } ,) ; violations . is_empty () . not () . then_some (violations) } fn all_macro_calls (& self , db : & dyn HirDatabase) -> Box < [(AstId < ast :: Item > , MacroCallId)] > { self . id . trait_items (db) . macro_calls . to_vec () . into_boxed_slice () } # [doc = " `#[rust_analyzer::completions(...)]` mode."] pub fn complete (self , db : & dyn HirDatabase) -> Complete { Complete :: extract (true , & self . attrs (db)) } }
    };
}

impl_305!()