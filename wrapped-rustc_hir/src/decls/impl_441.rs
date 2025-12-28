macro_rules! deps {
    () => {
        Variant!();
        PatKind!();
        Res!();
        PatExpr!();
        Pat!();
        Path!();
        QPath!();
        Expr!();
        DefKind!();
        CtorOf!();
        PatExprKind!();
    };
}

macro_rules! impl_441 {
    () => {
        deps!();
        impl hir :: Pat < '_ > { # [doc = " Call `f` on every \"binding\" in a pattern, e.g., on `a` in"] # [doc = " `match foo() { Some(a) => (), None => () }`"] pub fn each_binding (& self , mut f : impl FnMut (hir :: BindingMode , HirId , Span , Ident)) { self . walk_always (| p | { if let PatKind :: Binding (binding_mode , _ , ident , _) = p . kind { f (binding_mode , p . hir_id , p . span , ident) ; } }) ; } # [doc = " Call `f` on every \"binding\" in a pattern, e.g., on `a` in"] # [doc = " `match foo() { Some(a) => (), None => () }`."] # [doc = ""] # [doc = " When encountering an or-pattern `p_0 | ... | p_n` only the first non-never pattern will be"] # [doc = " visited. If they're all never patterns we visit nothing, which is ok since a never pattern"] # [doc = " cannot have bindings."] pub fn each_binding_or_first (& self , f : & mut impl FnMut (hir :: BindingMode , HirId , Span , Ident)) { self . walk (| p | match & p . kind { PatKind :: Or (ps) => { for p in * ps { if ! p . is_never_pattern () { p . each_binding_or_first (f) ; break ; } } false } PatKind :: Binding (bm , _ , ident , _) => { f (* bm , p . hir_id , p . span , * ident) ; true } _ => true , }) } pub fn simple_ident (& self) -> Option < Ident > { match self . kind { PatKind :: Binding (BindingMode (ByRef :: No , _) , _ , ident , None) => Some (ident) , _ => None , } } # [doc = " Returns variants that are necessary to exist for the pattern to match."] pub fn necessary_variants (& self) -> Vec < DefId > { let mut variants = vec ! [] ; self . walk (| p | match & p . kind { PatKind :: Or (_) => false , PatKind :: Expr (hir :: PatExpr { kind : hir :: PatExprKind :: Path (hir :: QPath :: Resolved (_ , path)) , .. }) | PatKind :: TupleStruct (hir :: QPath :: Resolved (_ , path) , ..) | PatKind :: Struct (hir :: QPath :: Resolved (_ , path) , ..) => { if let Res :: Def (DefKind :: Variant | DefKind :: Ctor (CtorOf :: Variant , ..) , id) = path . res { variants . push (id) ; } true } _ => true , }) ; let mut duplicates = DefIdSet :: default () ; variants . retain (| def_id | duplicates . insert (* def_id)) ; variants } # [doc = " Checks if the pattern contains any `ref` or `ref mut` bindings, and if"] # [doc = " yes whether it contains mutable or just immutables ones."] pub fn contains_explicit_ref_binding (& self) -> Option < hir :: Mutability > { let mut result = None ; self . each_binding (| annotation , _ , _ , _ | match annotation { hir :: BindingMode :: REF if result . is_none () => result = Some (hir :: Mutability :: Not) , hir :: BindingMode :: REF_MUT => result = Some (hir :: Mutability :: Mut) , _ => { } }) ; result } }
    };
}

impl_441!();