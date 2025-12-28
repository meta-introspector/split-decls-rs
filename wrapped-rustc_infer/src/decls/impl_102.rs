macro_rules! deps {
    () => {
        MatchAgainstHigherRankedOutlives!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < 'tcx > TypeRelation < TyCtxt < 'tcx > > for MatchAgainstHigherRankedOutlives < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } # [instrument (level = "trace" , skip (self))] fn relate_with_variance < T : Relate < TyCtxt < 'tcx > > > (& mut self , variance : ty :: Variance , _ : ty :: VarianceDiagInfo < TyCtxt < 'tcx > > , a : T , b : T ,) -> RelateResult < 'tcx , T > { if variance != ty :: Bivariant { self . relate (a , b) } else { Ok (a) } } # [instrument (skip (self) , level = "trace")] fn regions (& mut self , pattern : ty :: Region < 'tcx > , value : ty :: Region < 'tcx > ,) -> RelateResult < 'tcx , ty :: Region < 'tcx > > { if let ty :: RegionKind :: ReBound (depth , br) = pattern . kind () && depth == self . pattern_depth { self . bind (br , value) } else if pattern == value { Ok (pattern) } else { self . no_match () } } # [instrument (skip (self) , level = "trace")] fn tys (& mut self , pattern : Ty < 'tcx > , value : Ty < 'tcx >) -> RelateResult < 'tcx , Ty < 'tcx > > { if matches ! (pattern . kind () , ty :: Error (_) | ty :: Bound (..)) { self . no_match () } else if pattern == value { Ok (pattern) } else { relate :: structurally_relate_tys (self , pattern , value) } } # [instrument (skip (self) , level = "trace")] fn consts (& mut self , pattern : ty :: Const < 'tcx > , value : ty :: Const < 'tcx > ,) -> RelateResult < 'tcx , ty :: Const < 'tcx > > { if pattern == value { Ok (pattern) } else { relate :: structurally_relate_consts (self , pattern , value) } } # [instrument (skip (self) , level = "trace")] fn binders < T > (& mut self , pattern : ty :: Binder < 'tcx , T > , value : ty :: Binder < 'tcx , T > ,) -> RelateResult < 'tcx , ty :: Binder < 'tcx , T > > where T : Relate < TyCtxt < 'tcx > > , { self . pattern_depth . shift_in (1) ; let result = Ok (pattern . rebind (self . relate (pattern . skip_binder () , value . skip_binder ()) ?)) ; self . pattern_depth . shift_out (1) ; result } }
    };
}

impl_102!()