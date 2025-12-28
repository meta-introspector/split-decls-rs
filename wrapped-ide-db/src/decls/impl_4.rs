macro_rules! deps {
    () => {
        RootDatabase!();
        ActiveParameter!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 'db > ActiveParameter < 'db > { # [doc = " Returns information about the call argument this token is part of."] pub fn at_token (sema : & Semantics < 'db , RootDatabase > , token : SyntaxToken) -> Option < Self > { let (signature , active_parameter) = callable_for_token (sema , token) ? ; Self :: from_signature_and_active_parameter (sema , signature , active_parameter) } # [doc = " Returns information about the call argument this token is part of."] pub fn at_arg (sema : & 'db Semantics < 'db , RootDatabase > , list : ast :: ArgList , at : TextSize ,) -> Option < Self > { let (signature , active_parameter) = callable_for_arg_list (sema , list , at) ? ; Self :: from_signature_and_active_parameter (sema , signature , active_parameter) } fn from_signature_and_active_parameter (sema : & Semantics < 'db , RootDatabase > , signature : hir :: Callable < 'db > , active_parameter : Option < usize > ,) -> Option < Self > { let idx = active_parameter ? ; let mut params = signature . params () ; if idx >= params . len () { cov_mark :: hit ! (too_many_arguments) ; return None ; } let param = params . swap_remove (idx) ; Some (ActiveParameter { ty : param . ty () . clone () , src : sema . source (param) }) } pub fn ident (& self) -> Option < ast :: Name > { self . src . as_ref () . and_then (| param | match param . value . as_ref () . right () ? . pat () ? { ast :: Pat :: IdentPat (ident) => ident . name () , _ => None , }) } pub fn attrs (& self) -> Option < AstChildren < ast :: Attr > > { self . src . as_ref () . and_then (| param | Some (param . value . as_ref () . right () ? . attrs ())) } }
    };
}

impl_4!()