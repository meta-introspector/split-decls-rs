mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArg , IntrinsicDef , TyCtxt } ;}
mkuse!{use crate :: collector :: { MonoItems , create_fn_mono_item } ;}

macro_rules! collect_autodiff_fn_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_autodiff_fn in module {}", module_path!());
    };
}

mkfn!{
    collect_autodiff_fn_introspect!();
    pub (crate) fn collect_autodiff_fn < 'tcx > (tcx : TyCtxt < 'tcx > , instance : ty :: Instance < 'tcx > , intrinsic : IntrinsicDef , output : & mut MonoItems < 'tcx > ,) { if intrinsic . name != rustc_span :: sym :: autodiff { return ; } ; collect_autodiff_fn_from_arg (instance . args [0] , tcx , output) ; }
}

macro_rules! collect_autodiff_fn_from_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_autodiff_fn_from_arg in module {}", module_path!());
    };
}

mkfn!{
    collect_autodiff_fn_from_arg_introspect!();
    fn collect_autodiff_fn_from_arg < 'tcx > (arg : GenericArg < 'tcx > , tcx : TyCtxt < 'tcx > , output : & mut MonoItems < 'tcx > ,) { let (instance , span) = match arg . kind () { ty :: GenericArgKind :: Type (ty) => match ty . kind () { ty :: FnDef (def_id , substs) => { let span = tcx . def_span (def_id) ; let instance = ty :: Instance :: expect_resolve (tcx , ty :: TypingEnv :: non_body_analysis (tcx , def_id) , * def_id , substs , span ,) ; (instance , span) } _ => bug ! ("expected autodiff function") , } , _ => bug ! ("expected type when matching autodiff arg") , } ; output . push (create_fn_mono_item (tcx , instance , span)) ; }
}