macro_rules! deps {
    () => {
        Ty!();
        PatKind!();
        BindingMode!();
        Expr!();
        MutTy!();
        Path!();
        TyKind!();
        MacCall!();
        Pat!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl Pat { # [doc = " Attempt reparsing the pattern as a type."] # [doc = " This is intended for use by diagnostics."] pub fn to_ty (& self) -> Option < Box < Ty > > { let kind = match & self . kind { PatKind :: Missing => unreachable ! () , PatKind :: Wild => TyKind :: Infer , PatKind :: Ident (BindingMode :: NONE , ident , None) => { TyKind :: Path (None , Path :: from_ident (* ident)) } PatKind :: Path (qself , path) => TyKind :: Path (qself . clone () , path . clone ()) , PatKind :: MacCall (mac) => TyKind :: MacCall (mac . clone ()) , PatKind :: Ref (pat , mutbl) => { pat . to_ty () . map (| ty | TyKind :: Ref (None , MutTy { ty , mutbl : * mutbl })) ? } PatKind :: Slice (pats) if let [pat] = pats . as_slice () => { pat . to_ty () . map (TyKind :: Slice) ? } PatKind :: Tuple (pats) => { let mut tys = ThinVec :: with_capacity (pats . len ()) ; for pat in pats { tys . push (pat . to_ty () ?) ; } TyKind :: Tup (tys) } _ => return None , } ; Some (Box :: new (Ty { kind , id : self . id , span : self . span , tokens : None })) } # [doc = " Walk top-down and call `it` in each place where a pattern occurs"] # [doc = " starting with the root pattern `walk` is called on. If `it` returns"] # [doc = " false then we will descend no further but siblings will be processed."] pub fn walk < 'ast > (& 'ast self , it : & mut impl FnMut (& 'ast Pat) -> bool) { if ! it (self) { return ; } match & self . kind { PatKind :: Ident (_ , _ , Some (p)) => p . walk (it) , PatKind :: Struct (_ , _ , fields , _) => fields . iter () . for_each (| field | field . pat . walk (it)) , PatKind :: TupleStruct (_ , _ , s) | PatKind :: Tuple (s) | PatKind :: Slice (s) | PatKind :: Or (s) => s . iter () . for_each (| p | p . walk (it)) , PatKind :: Box (s) | PatKind :: Deref (s) | PatKind :: Ref (s , _) | PatKind :: Paren (s) | PatKind :: Guard (s , _) => s . walk (it) , PatKind :: Missing | PatKind :: Wild | PatKind :: Rest | PatKind :: Never | PatKind :: Expr (_) | PatKind :: Range (..) | PatKind :: Ident (..) | PatKind :: Path (..) | PatKind :: MacCall (_) | PatKind :: Err (_) => { } } } # [doc = " Is this a `..` pattern?"] pub fn is_rest (& self) -> bool { matches ! (self . kind , PatKind :: Rest) } # [doc = " Whether this could be a never pattern, taking into account that a macro invocation can"] # [doc = " return a never pattern. Used to inform errors during parsing."] pub fn could_be_never_pattern (& self) -> bool { let mut could_be_never_pattern = false ; self . walk (& mut | pat | match & pat . kind { PatKind :: Never | PatKind :: MacCall (_) => { could_be_never_pattern = true ; false } PatKind :: Or (s) => { could_be_never_pattern = s . iter () . all (| p | p . could_be_never_pattern ()) ; false } _ => true , }) ; could_be_never_pattern } # [doc = " Whether this contains a `!` pattern. This in particular means that a feature gate error will"] # [doc = " be raised if the feature is off. Used to avoid gating the feature twice."] pub fn contains_never_pattern (& self) -> bool { let mut contains_never_pattern = false ; self . walk (& mut | pat | { if matches ! (pat . kind , PatKind :: Never) { contains_never_pattern = true ; } true }) ; contains_never_pattern } # [doc = " Return a name suitable for diagnostics."] pub fn descr (& self) -> Option < String > { match & self . kind { PatKind :: Missing => unreachable ! () , PatKind :: Wild => Some ("_" . to_string ()) , PatKind :: Ident (BindingMode :: NONE , ident , None) => Some (format ! ("{ident}")) , PatKind :: Ref (pat , mutbl) => pat . descr () . map (| d | format ! ("&{}{d}" , mutbl . prefix_str ())) , _ => None , } } }
    };
}

impl_51!()