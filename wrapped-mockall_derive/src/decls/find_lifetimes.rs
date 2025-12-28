macro_rules! deps {
    () => {
        HashSet!();
    };
}

macro_rules! find_lifetimes {
    () => {
        deps!();
        fn find_lifetimes (ty : & Type) -> HashSet < Lifetime > { match ty { Type :: Array (ta) => find_lifetimes (ta . elem . as_ref ()) , Type :: Group (tg) => find_lifetimes (tg . elem . as_ref ()) , Type :: Infer (_ti) => HashSet :: default () , Type :: Never (_tn) => HashSet :: default () , Type :: Paren (tp) => find_lifetimes (tp . elem . as_ref ()) , Type :: Path (tp) => { let mut ret = find_lifetimes_in_path (& tp . path) ; if let Some (qs) = & tp . qself { ret . extend (find_lifetimes (qs . ty . as_ref ())) ; } ret } , Type :: Ptr (tp) => find_lifetimes (tp . elem . as_ref ()) , Type :: Reference (tr) => { let mut ret = find_lifetimes (tr . elem . as_ref ()) ; if let Some (lt) = & tr . lifetime { ret . insert (lt . clone ()) ; } ret } , Type :: Slice (ts) => find_lifetimes (ts . elem . as_ref ()) , Type :: TraitObject (tto) => { let mut ret = HashSet :: default () ; for bound in tto . bounds . iter () { ret . extend (find_lifetimes_in_tpb (bound)) ; } ret } Type :: Tuple (tt) => { let mut ret = HashSet :: default () ; for ty in tt . elems . iter () { ret . extend (find_lifetimes (ty)) ; } ret } , Type :: ImplTrait (tit) => { let mut ret = HashSet :: default () ; for tpb in tit . bounds . iter () { ret . extend (find_lifetimes_in_tpb (tpb)) ; } ret } , _ => { compile_error (ty . span () , "unsupported type in this context") ; HashSet :: default () } } }
    };
}

find_lifetimes!()