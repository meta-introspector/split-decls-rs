macro_rules! deps {
    () => {
        TypeParam!();
    };
}

macro_rules! write_where_predicates {
    () => {
        deps!();
        fn write_where_predicates < 'db > (params : & GenericParams , store : & ExpressionStore , f : & mut HirFormatter < '_ , 'db > ,) -> Result < () , HirDisplayError > { use WherePredicate :: * ; let is_unnamed_type_target = | target : TypeRefId | { matches ! (store [target] , TypeRef :: TypeParam (id) if f . db . generic_params (id . parent ()) [id . local_id ()] . name () . is_none ()) } ; let check_same_target = | pred1 : & WherePredicate , pred2 : & WherePredicate | match (pred1 , pred2) { (TypeBound { target : t1 , .. } , TypeBound { target : t2 , .. }) => t1 == t2 , (Lifetime { target : t1 , .. } , Lifetime { target : t2 , .. }) => t1 == t2 , (ForLifetime { lifetimes : l1 , target : t1 , .. } , ForLifetime { lifetimes : l2 , target : t2 , .. } ,) => l1 == l2 && t1 == t2 , _ => false , } ; let mut iter = params . where_predicates () . iter () . peekable () ; while let Some (pred) = iter . next () { if matches ! (pred , TypeBound { target , .. } if is_unnamed_type_target (* target)) { continue ; } f . write_str ("\n    ") ? ; match pred { TypeBound { target , bound } => { target . hir_fmt (f , store) ? ; f . write_str (": ") ? ; bound . hir_fmt (f , store) ? ; } Lifetime { target , bound } => { target . hir_fmt (f , store) ? ; write ! (f , ": ") ? ; bound . hir_fmt (f , store) ? ; } ForLifetime { lifetimes , target , bound } => { let lifetimes = lifetimes . iter () . map (| it | it . display (f . db , f . edition ())) . join (", ") ; write ! (f , "for<{lifetimes}> ") ? ; target . hir_fmt (f , store) ? ; f . write_str (": ") ? ; bound . hir_fmt (f , store) ? ; } } while let Some (nxt) = iter . next_if (| nxt | check_same_target (pred , nxt)) { f . write_str (" + ") ? ; match nxt { TypeBound { bound , .. } | ForLifetime { bound , .. } => bound . hir_fmt (f , store) ? , Lifetime { bound , .. } => bound . hir_fmt (f , store) ? , } } f . write_str (",") ? ; } Ok (()) }
    };
}

write_where_predicates!();