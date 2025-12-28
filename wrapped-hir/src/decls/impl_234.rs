macro_rules! deps {
    () => {
        GenericDef!();
        DefWithBody!();
        ModuleDef!();
        BuiltinType!();
        Macro!();
        Trait!();
        Enum!();
        Adt!();
        Union!();
        Const!();
        TypeAlias!();
        Function!();
        Struct!();
        Variant!();
        Module!();
        Static!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl ModuleDef { pub fn module (self , db : & dyn HirDatabase) -> Option < Module > { match self { ModuleDef :: Module (it) => it . parent (db) , ModuleDef :: Function (it) => Some (it . module (db)) , ModuleDef :: Adt (it) => Some (it . module (db)) , ModuleDef :: Variant (it) => Some (it . module (db)) , ModuleDef :: Const (it) => Some (it . module (db)) , ModuleDef :: Static (it) => Some (it . module (db)) , ModuleDef :: Trait (it) => Some (it . module (db)) , ModuleDef :: TypeAlias (it) => Some (it . module (db)) , ModuleDef :: Macro (it) => Some (it . module (db)) , ModuleDef :: BuiltinType (_) => None , } } pub fn canonical_path (& self , db : & dyn HirDatabase , edition : Edition) -> Option < String > { let mut segments = vec ! [self . name (db) ?] ; for m in self . module (db) ? . path_to_root (db) { segments . extend (m . name (db)) } segments . reverse () ; Some (segments . iter () . map (| it | it . display (db , edition)) . join ("::")) } pub fn canonical_module_path (& self , db : & dyn HirDatabase ,) -> Option < impl Iterator < Item = Module > > { self . module (db) . map (| it | it . path_to_root (db) . into_iter () . rev ()) } pub fn name (self , db : & dyn HirDatabase) -> Option < Name > { let name = match self { ModuleDef :: Module (it) => it . name (db) ? , ModuleDef :: Const (it) => it . name (db) ? , ModuleDef :: Adt (it) => it . name (db) , ModuleDef :: Trait (it) => it . name (db) , ModuleDef :: Function (it) => it . name (db) , ModuleDef :: Variant (it) => it . name (db) , ModuleDef :: TypeAlias (it) => it . name (db) , ModuleDef :: Static (it) => it . name (db) , ModuleDef :: Macro (it) => it . name (db) , ModuleDef :: BuiltinType (it) => it . name () , } ; Some (name) } pub fn diagnostics < 'db > (self , db : & 'db dyn HirDatabase , style_lints : bool ,) -> Vec < AnyDiagnostic < 'db > > { let id = match self { ModuleDef :: Adt (it) => match it { Adt :: Struct (it) => it . id . into () , Adt :: Enum (it) => it . id . into () , Adt :: Union (it) => it . id . into () , } , ModuleDef :: Trait (it) => it . id . into () , ModuleDef :: Function (it) => it . id . into () , ModuleDef :: TypeAlias (it) => it . id . into () , ModuleDef :: Module (it) => it . id . into () , ModuleDef :: Const (it) => it . id . into () , ModuleDef :: Static (it) => it . id . into () , ModuleDef :: Variant (it) => it . id . into () , ModuleDef :: BuiltinType (_) | ModuleDef :: Macro (_) => return Vec :: new () , } ; let mut acc = Vec :: new () ; match self . as_def_with_body () { Some (def) => { def . diagnostics (db , & mut acc , style_lints) ; } None => { for diag in hir_ty :: diagnostics :: incorrect_case (db , id) { acc . push (diag . into ()) } } } if let Some (def) = self . as_self_generic_def () { def . diagnostics (db , & mut acc) ; } acc } pub fn as_def_with_body (self) -> Option < DefWithBody > { match self { ModuleDef :: Function (it) => Some (it . into ()) , ModuleDef :: Const (it) => Some (it . into ()) , ModuleDef :: Static (it) => Some (it . into ()) , ModuleDef :: Variant (it) => Some (it . into ()) , ModuleDef :: Module (_) | ModuleDef :: Adt (_) | ModuleDef :: Trait (_) | ModuleDef :: TypeAlias (_) | ModuleDef :: Macro (_) | ModuleDef :: BuiltinType (_) => None , } } # [doc = " Returns only defs that have generics from themselves, not their parent."] pub fn as_self_generic_def (self) -> Option < GenericDef > { match self { ModuleDef :: Function (it) => Some (it . into ()) , ModuleDef :: Adt (it) => Some (it . into ()) , ModuleDef :: Trait (it) => Some (it . into ()) , ModuleDef :: TypeAlias (it) => Some (it . into ()) , ModuleDef :: Module (_) | ModuleDef :: Variant (_) | ModuleDef :: Static (_) | ModuleDef :: Const (_) | ModuleDef :: BuiltinType (_) | ModuleDef :: Macro (_) => None , } } pub fn attrs (& self , db : & dyn HirDatabase) -> Option < AttrsWithOwner > { Some (match self { ModuleDef :: Module (it) => it . attrs (db) , ModuleDef :: Function (it) => it . attrs (db) , ModuleDef :: Adt (it) => it . attrs (db) , ModuleDef :: Variant (it) => it . attrs (db) , ModuleDef :: Const (it) => it . attrs (db) , ModuleDef :: Static (it) => it . attrs (db) , ModuleDef :: Trait (it) => it . attrs (db) , ModuleDef :: TypeAlias (it) => it . attrs (db) , ModuleDef :: Macro (it) => it . attrs (db) , ModuleDef :: BuiltinType (_) => return None , }) } }
    };
}

impl_234!();