macro_rules! deps {
    () => {
        GenericParamAndBoundVarCollector!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for GenericParamAndBoundVarCollector < '_ , 'tcx > { type Result = ControlFlow < ErrorGuaranteed > ; fn visit_binder < T : TypeVisitable < TyCtxt < 'tcx > > > (& mut self , binder : & ty :: Binder < 'tcx , T > ,) -> Self :: Result { self . depth . shift_in (1) ; let result = binder . super_visit_with (self) ; self . depth . shift_out (1) ; result } fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { match ty . kind () { ty :: Param (param) => { self . params . insert (param . index) ; } ty :: Bound (db , bt) if * db >= self . depth => { self . vars . insert (match bt . kind { ty :: BoundTyKind :: Param (def_id) => def_id , ty :: BoundTyKind :: Anon => { let reported = self . cx . dcx () . delayed_bug (format ! ("unexpected anon bound ty: {:?}" , bt . var)) ; return ControlFlow :: Break (reported) ; } }) ; } _ if ty . has_param () || ty . has_bound_vars () => return ty . super_visit_with (self) , _ => { } } ControlFlow :: Continue (()) } fn visit_region (& mut self , re : ty :: Region < 'tcx >) -> Self :: Result { match re . kind () { ty :: ReEarlyParam (param) => { self . params . insert (param . index) ; } ty :: ReBound (db , br) if db >= self . depth => { self . vars . insert (match br . kind { ty :: BoundRegionKind :: Named (def_id) => def_id , ty :: BoundRegionKind :: Anon | ty :: BoundRegionKind :: ClosureEnv => { let guar = self . cx . dcx () . delayed_bug (format ! ("unexpected bound region kind: {:?}" , br . kind)) ; return ControlFlow :: Break (guar) ; } ty :: BoundRegionKind :: NamedAnon (_) => bug ! ("only used for pretty printing") , }) ; } _ => { } } ControlFlow :: Continue (()) } fn visit_const (& mut self , ct : ty :: Const < 'tcx >) -> Self :: Result { match ct . kind () { ty :: ConstKind :: Param (param) => { self . params . insert (param . index) ; } ty :: ConstKind :: Bound (db , _) if db >= self . depth => { let guar = self . cx . dcx () . delayed_bug ("unexpected escaping late-bound const var") ; return ControlFlow :: Break (guar) ; } _ if ct . has_param () || ct . has_bound_vars () => return ct . super_visit_with (self) , _ => { } } ControlFlow :: Continue (()) } }
    };
}

impl_438!()