macro_rules! deps {
    () => {
        VarianceExtractor!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl < 'tcx > TypeRelation < TyCtxt < 'tcx > > for VarianceExtractor < '_ , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn relate_with_variance < T : Relate < TyCtxt < 'tcx > > > (& mut self , variance : ty :: Variance , _info : ty :: VarianceDiagInfo < TyCtxt < 'tcx > > , a : T , b : T ,) -> RelateResult < 'tcx , T > { let old_ambient_variance = self . ambient_variance ; self . ambient_variance = self . ambient_variance . xform (variance) ; let r = self . relate (a , b) ? ; self . ambient_variance = old_ambient_variance ; Ok (r) } fn tys (& mut self , a : Ty < 'tcx > , b : Ty < 'tcx >) -> RelateResult < 'tcx , Ty < 'tcx > > { assert_eq ! (a , b) ; relate :: structurally_relate_tys (self , a , b) } fn regions (& mut self , a : ty :: Region < 'tcx > , b : ty :: Region < 'tcx > ,) -> RelateResult < 'tcx , ty :: Region < 'tcx > > { assert_eq ! (a , b) ; self . record_variance (a , self . ambient_variance) ; Ok (a) } fn consts (& mut self , a : ty :: Const < 'tcx > , b : ty :: Const < 'tcx > ,) -> RelateResult < 'tcx , ty :: Const < 'tcx > > { assert_eq ! (a , b) ; relate :: structurally_relate_consts (self , a , b) } fn binders < T > (& mut self , a : ty :: Binder < 'tcx , T > , _ : ty :: Binder < 'tcx , T > ,) -> RelateResult < 'tcx , ty :: Binder < 'tcx , T > > where T : Relate < TyCtxt < 'tcx > > , { self . relate (a . skip_binder () , a . skip_binder ()) ? ; Ok (a) } }
    };
}

impl_287!();