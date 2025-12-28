macro_rules! deps {
    () => {
        FunctionalVariances!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl < 'tcx > TypeRelation < TyCtxt < 'tcx > > for FunctionalVariances < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn relate_with_variance < T : Relate < TyCtxt < 'tcx > > > (& mut self , variance : ty :: Variance , _ : ty :: VarianceDiagInfo < TyCtxt < 'tcx > > , a : T , b : T ,) -> RelateResult < 'tcx , T > { let old_variance = self . ambient_variance ; self . ambient_variance = self . ambient_variance . xform (variance) ; self . relate (a , b) . unwrap () ; self . ambient_variance = old_variance ; Ok (a) } fn tys (& mut self , a : Ty < 'tcx > , b : Ty < 'tcx >) -> RelateResult < 'tcx , Ty < 'tcx > > { structurally_relate_tys (self , a , b) . unwrap () ; Ok (a) } fn regions (& mut self , a : ty :: Region < 'tcx > , _ : ty :: Region < 'tcx > ,) -> RelateResult < 'tcx , ty :: Region < 'tcx > > { let def_id = match a . kind () { ty :: ReEarlyParam (ebr) => self . generics . region_param (ebr , self . tcx) . def_id , ty :: ReBound (_ , ty :: BoundRegion { kind : ty :: BoundRegionKind :: Named (def_id) , .. }) | ty :: ReLateParam (ty :: LateParamRegion { scope : _ , kind : ty :: LateParamRegionKind :: Named (def_id) , }) => def_id , _ => { return Ok (a) ; } } ; if let Some (variance) = self . variances . get_mut (& def_id) { * variance = unify (* variance , self . ambient_variance) ; } else { self . variances . insert (def_id , self . ambient_variance) ; } Ok (a) } fn consts (& mut self , a : ty :: Const < 'tcx > , b : ty :: Const < 'tcx > ,) -> RelateResult < 'tcx , ty :: Const < 'tcx > > { structurally_relate_consts (self , a , b) . unwrap () ; Ok (a) } fn binders < T > (& mut self , a : ty :: Binder < 'tcx , T > , b : ty :: Binder < 'tcx , T > ,) -> RelateResult < 'tcx , ty :: Binder < 'tcx , T > > where T : Relate < TyCtxt < 'tcx > > , { self . relate (a . skip_binder () , b . skip_binder ()) . unwrap () ; Ok (a) } }
    };
}

impl_260!();