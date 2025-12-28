macro_rules! deps {
    () => {
        AngleBrackets!();
        Path!();
        Lifetime!();
        LifetimeSyntax!();
        LifetimeSource!();
        LifetimeKind!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl Lifetime { pub fn new (hir_id : HirId , ident : Ident , kind : LifetimeKind , source : LifetimeSource , syntax : LifetimeSyntax ,) -> Lifetime { let lifetime = Lifetime { hir_id , ident , kind , source , syntax } ; # [cfg (debug_assertions)] match (lifetime . is_elided () , lifetime . is_anonymous ()) { (false , false) => { } (false , true) => { } (true , true) => { } (true , false) => panic ! ("bad Lifetime") , } lifetime } pub fn is_elided (& self) -> bool { self . kind . is_elided () } pub fn is_anonymous (& self) -> bool { self . ident . name == kw :: UnderscoreLifetime } pub fn is_implicit (& self) -> bool { matches ! (self . syntax , LifetimeSyntax :: Implicit) } pub fn is_static (& self) -> bool { self . kind == LifetimeKind :: Static } pub fn suggestion (& self , new_lifetime : & str) -> (Span , String) { use LifetimeSource :: * ; use LifetimeSyntax :: * ; debug_assert ! (new_lifetime . starts_with ('\'')) ; match (self . syntax , self . source) { (ExplicitBound | ExplicitAnonymous , _) => (self . ident . span , format ! ("{new_lifetime}")) , (Implicit , Path { angle_brackets : AngleBrackets :: Full }) => { (self . ident . span , format ! ("{new_lifetime}, ")) } (Implicit , Path { angle_brackets : AngleBrackets :: Empty }) => { (self . ident . span , format ! ("{new_lifetime}")) } (Implicit , Path { angle_brackets : AngleBrackets :: Missing }) => { (self . ident . span . shrink_to_hi () , format ! ("<{new_lifetime}>")) } (Implicit , Reference) => (self . ident . span , format ! ("{new_lifetime} ")) , (Implicit , source) => { unreachable ! ("can't suggest for a implicit lifetime of {source:?}") } } } }
    };
}

impl_113!();