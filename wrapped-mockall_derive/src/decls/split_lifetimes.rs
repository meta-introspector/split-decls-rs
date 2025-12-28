macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! split_lifetimes {
    () => {
        deps!();
        # [doc = " Split a generics list into three: one for type generics and where predicates"] # [doc = " that relate to the signature, one for lifetimes that relate to the arguments"] # [doc = " only, and one for lifetimes that relate to the return type only."] fn split_lifetimes (generics : Generics , args : & Punctuated < FnArg , Token ! [,] > , rt : & ReturnType) -> (Generics , Punctuated < LifetimeParam , token :: Comma > , Punctuated < LifetimeParam , token :: Comma >) { if generics . lt_token . is_none () { return (generics , Default :: default () , Default :: default ()) ; } let mut alts = HashSet :: < Lifetime > :: default () ; let mut rlts = HashSet :: < Lifetime > :: default () ; for arg in args { match arg { FnArg :: Receiver (r) => { if let Some ((_ , Some (lt))) = & r . reference { alts . insert (lt . clone ()) ; } } , FnArg :: Typed (pt) => { alts . extend (find_lifetimes (pt . ty . as_ref ())) ; } , } ; } ; if let ReturnType :: Type (_ , ty) = rt { rlts . extend (find_lifetimes (ty)) ; } let mut tv = Punctuated :: new () ; let mut alv = Punctuated :: new () ; let mut rlv = Punctuated :: new () ; for p in generics . params . into_iter () { match p { GenericParam :: Lifetime (ltd) if rlts . contains (& ltd . lifetime) => rlv . push (ltd) , GenericParam :: Lifetime (ltd) if alts . contains (& ltd . lifetime) => alv . push (ltd) , GenericParam :: Lifetime (_) => { } , GenericParam :: Type (_) => tv . push (p) , _ => () , } } let tg = if tv . is_empty () { Generics :: default () } else { Generics { lt_token : generics . lt_token , gt_token : generics . gt_token , params : tv , where_clause : generics . where_clause } } ; (tg , alv , rlv) }
    };
}

split_lifetimes!()