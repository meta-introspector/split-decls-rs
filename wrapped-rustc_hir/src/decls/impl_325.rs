macro_rules! deps {
    () => {
        ItemKind!();
        UseKind!();
        Impl!();
        Generics!();
        Mod!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl ItemKind < '_ > { pub fn ident (& self) -> Option < Ident > { match * self { ItemKind :: ExternCrate (_ , ident) | ItemKind :: Use (_ , UseKind :: Single (ident)) | ItemKind :: Static (_ , ident , ..) | ItemKind :: Const (ident , ..) | ItemKind :: Fn { ident , .. } | ItemKind :: Macro (ident , ..) | ItemKind :: Mod (ident , ..) | ItemKind :: TyAlias (ident , ..) | ItemKind :: Enum (ident , ..) | ItemKind :: Struct (ident , ..) | ItemKind :: Union (ident , ..) | ItemKind :: Trait (_ , _ , _ , ident , ..) | ItemKind :: TraitAlias (ident , ..) => Some (ident) , ItemKind :: Use (_ , UseKind :: Glob | UseKind :: ListStem) | ItemKind :: ForeignMod { .. } | ItemKind :: GlobalAsm { .. } | ItemKind :: Impl (_) => None , } } pub fn generics (& self) -> Option < & Generics < '_ > > { Some (match self { ItemKind :: Fn { generics , .. } | ItemKind :: TyAlias (_ , generics , _) | ItemKind :: Const (_ , generics , _ , _) | ItemKind :: Enum (_ , generics , _) | ItemKind :: Struct (_ , generics , _) | ItemKind :: Union (_ , generics , _) | ItemKind :: Trait (_ , _ , _ , _ , generics , _ , _) | ItemKind :: TraitAlias (_ , generics , _) | ItemKind :: Impl (Impl { generics , .. }) => generics , _ => return None , }) } }
    };
}

impl_325!();