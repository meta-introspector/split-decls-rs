macro_rules! deps {
    () => {
        InheritanceKind!();
    };
}

macro_rules! build_predicates {
    () => {
        deps!();
        fn build_predicates < 'tcx > (tcx : TyCtxt < 'tcx > , sig_id : DefId , parent : Option < DefId > , inh_kind : InheritanceKind , args : ty :: GenericArgsRef < 'tcx > ,) -> ty :: GenericPredicates < 'tcx > { struct PredicatesCollector < 'tcx > { tcx : TyCtxt < 'tcx > , preds : Vec < (ty :: Clause < 'tcx > , Span) > , args : ty :: GenericArgsRef < 'tcx > , } impl < 'tcx > PredicatesCollector < 'tcx > { fn new (tcx : TyCtxt < 'tcx > , args : ty :: GenericArgsRef < 'tcx >) -> PredicatesCollector < 'tcx > { PredicatesCollector { tcx , preds : vec ! [] , args } } fn with_own_preds (mut self , f : impl Fn (DefId) -> ty :: GenericPredicates < 'tcx > , def_id : DefId ,) -> Self { let preds = f (def_id) . instantiate_own (self . tcx , self . args) ; self . preds . extend (preds) ; self } fn with_preds (mut self , f : impl Fn (DefId) -> ty :: GenericPredicates < 'tcx > + Copy , def_id : DefId ,) -> Self { let preds = f (def_id) ; if let Some (parent_def_id) = preds . parent { self = self . with_own_preds (f , parent_def_id) ; } self . with_own_preds (f , def_id) } } let collector = PredicatesCollector :: new (tcx , args) ; let preds = match inh_kind { InheritanceKind :: WithParent (false) => { collector . with_preds (| def_id | tcx . explicit_predicates_of (def_id) , sig_id) } InheritanceKind :: WithParent (true) => { collector . with_preds (| def_id | tcx . predicates_of (def_id) , sig_id) } InheritanceKind :: Own => { collector . with_own_preds (| def_id | tcx . predicates_of (def_id) , sig_id) } } . preds ; ty :: GenericPredicates { parent , predicates : tcx . arena . alloc_from_iter (preds) } }
    };
}

build_predicates!();