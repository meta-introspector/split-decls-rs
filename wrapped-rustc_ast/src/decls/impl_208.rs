macro_rules! deps {
    () => {
        ForeignMod!();
        Fn!();
        ItemKind!();
        Trait!();
        DelegationMac!();
        Const!();
        Impl!();
        Generics!();
        Item!();
        MacCall!();
        Delegation!();
        TyAlias!();
        MacroDef!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl Item { # [doc = " Return the span that encompasses the attributes."] pub fn span_with_attributes (& self) -> Span { self . attrs . iter () . fold (self . span , | acc , attr | acc . to (attr . span)) } pub fn opt_generics (& self) -> Option < & Generics > { match & self . kind { ItemKind :: ExternCrate (..) | ItemKind :: Use (_) | ItemKind :: Mod (..) | ItemKind :: ForeignMod (_) | ItemKind :: GlobalAsm (_) | ItemKind :: MacCall (_) | ItemKind :: Delegation (_) | ItemKind :: DelegationMac (_) | ItemKind :: MacroDef (..) => None , ItemKind :: Static (_) => None , ItemKind :: Const (i) => Some (& i . generics) , ItemKind :: Fn (i) => Some (& i . generics) , ItemKind :: TyAlias (i) => Some (& i . generics) , ItemKind :: TraitAlias (_ , generics , _) | ItemKind :: Enum (_ , generics , _) | ItemKind :: Struct (_ , generics , _) | ItemKind :: Union (_ , generics , _) => Some (& generics) , ItemKind :: Trait (i) => Some (& i . generics) , ItemKind :: Impl (i) => Some (& i . generics) , } } }
    };
}

impl_208!()