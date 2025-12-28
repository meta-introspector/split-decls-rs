macro_rules! deps {
    () => {
        LatticeOp!();
        InferCtxt!();
        Obligation!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < 'tcx > PredicateEmittingRelation < InferCtxt < 'tcx > > for LatticeOp < '_ , 'tcx > { fn span (& self) -> Span { self . trace . span () } fn structurally_relate_aliases (& self) -> StructurallyRelateAliases { StructurallyRelateAliases :: No } fn param_env (& self) -> ty :: ParamEnv < 'tcx > { self . param_env } fn register_predicates (& mut self , preds : impl IntoIterator < Item : ty :: Upcast < TyCtxt < 'tcx > , ty :: Predicate < 'tcx > > > ,) { self . obligations . extend (preds . into_iter () . map (| pred | { Obligation :: new (self . infcx . tcx , self . trace . cause . clone () , self . param_env , pred) })) } fn register_goals (& mut self , goals : impl IntoIterator < Item = Goal < 'tcx , ty :: Predicate < 'tcx > > >) { self . obligations . extend (goals . into_iter () . map (| goal | { Obligation :: new (self . infcx . tcx , self . trace . cause . clone () , goal . param_env , goal . predicate ,) })) } fn register_alias_relate_predicate (& mut self , a : Ty < 'tcx > , b : Ty < 'tcx >) { self . register_predicates ([ty :: Binder :: dummy (ty :: PredicateKind :: AliasRelate (a . into () , b . into () , ty :: AliasRelationDirection :: Equate ,))]) ; } }
    };
}

impl_169!()