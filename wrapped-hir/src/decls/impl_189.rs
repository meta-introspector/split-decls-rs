macro_rules! deps {
    () => {
        ModuleDef!();
        GenericParam!();
        ScopeDef!();
        Label!();
        Local!();
        Crate!();
        Macro!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl ScopeDef { pub fn all_items (def : PerNs) -> ArrayVec < Self , 3 > { let mut items = ArrayVec :: new () ; match (def . take_types () , def . take_values ()) { (Some (m1) , None) => items . push (ScopeDef :: ModuleDef (m1 . into ())) , (None , Some (m2)) => items . push (ScopeDef :: ModuleDef (m2 . into ())) , (Some (m1) , Some (m2)) => { if m1 != m2 { items . push (ScopeDef :: ModuleDef (m1 . into ())) ; items . push (ScopeDef :: ModuleDef (m2 . into ())) ; } else { items . push (ScopeDef :: ModuleDef (m1 . into ())) ; } } (None , None) => { } } ; if let Some (macro_def_id) = def . take_macros () { items . push (ScopeDef :: ModuleDef (ModuleDef :: Macro (macro_def_id . into ()))) ; } if items . is_empty () { items . push (ScopeDef :: Unknown) ; } items } pub fn attrs (& self , db : & dyn HirDatabase) -> Option < AttrsWithOwner > { match self { ScopeDef :: ModuleDef (it) => it . attrs (db) , ScopeDef :: GenericParam (it) => Some (it . attrs (db)) , ScopeDef :: ImplSelfType (_) | ScopeDef :: AdtSelfType (_) | ScopeDef :: Local (_) | ScopeDef :: Label (_) | ScopeDef :: Unknown => None , } } pub fn krate (& self , db : & dyn HirDatabase) -> Option < Crate > { match self { ScopeDef :: ModuleDef (it) => it . module (db) . map (| m | m . krate ()) , ScopeDef :: GenericParam (it) => Some (it . module (db) . krate ()) , ScopeDef :: ImplSelfType (_) => None , ScopeDef :: AdtSelfType (it) => Some (it . module (db) . krate ()) , ScopeDef :: Local (it) => Some (it . module (db) . krate ()) , ScopeDef :: Label (it) => Some (it . module (db) . krate ()) , ScopeDef :: Unknown => None , } } }
    };
}

impl_189!()