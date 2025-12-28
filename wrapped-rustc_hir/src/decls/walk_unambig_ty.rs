macro_rules! deps {
    () => {
        Ty!();
        Visitor!();
        InferKind!();
    };
}

macro_rules! walk_unambig_ty {
    () => {
        deps!();
        pub fn walk_unambig_ty < 'v , V : Visitor < 'v > > (visitor : & mut V , typ : & 'v Ty < 'v >) -> V :: Result { match typ . try_as_ambig_ty () { Some (ambig_ty) => visitor . visit_ty (ambig_ty) , None => { let Ty { hir_id , span , kind : _ } = typ ; visitor . visit_infer (* hir_id , * span , InferKind :: Ty (typ)) } } }
    };
}

walk_unambig_ty!();