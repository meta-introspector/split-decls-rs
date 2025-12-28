macro_rules! deps {
    () => {
        MetaVarKind!();
        Expr!();
        Lit!();
        Token!();
        InvisibleOrigin!();
        IdentIsRaw!();
        LitKind!();
    };
}

macro_rules! impl_400 {
    () => {
        deps!();
        impl Lit { pub fn new (kind : LitKind , symbol : Symbol , suffix : Option < Symbol >) -> Lit { Lit { kind , symbol , suffix } } # [doc = " Returns `true` if this is semantically a float literal. This includes"] # [doc = " ones like `1f32` that have an `Integer` kind but a float suffix."] pub fn is_semantic_float (& self) -> bool { match self . kind { LitKind :: Float => true , LitKind :: Integer => match self . suffix { Some (sym) => sym == sym :: f32 || sym == sym :: f64 , None => false , } , _ => false , } } # [doc = " Keep this in sync with `Token::can_begin_literal_maybe_minus` and"] # [doc = " `Parser::eat_token_lit` (excluding unary negation)."] pub fn from_token (token : & Token) -> Option < Lit > { match token . uninterpolate () . kind { Ident (name , IdentIsRaw :: No) if name . is_bool_lit () => Some (Lit :: new (Bool , name , None)) , Literal (token_lit) => Some (token_lit) , OpenInvisible (InvisibleOrigin :: MetaVar (MetaVarKind :: Literal | MetaVarKind :: Expr { .. } ,)) => { panic ! ("from_token metavar") ; } _ => None , } } }
    };
}

impl_400!();