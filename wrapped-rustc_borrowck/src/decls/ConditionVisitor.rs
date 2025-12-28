macro_rules! ConditionVisitor {
    () => {
        # [doc = " Given a set of spans representing statements initializing the relevant binding, visit all the"] # [doc = " function expressions looking for branching code paths that *do not* initialize the binding."] struct ConditionVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , spans : Vec < Span > , name : String , errors : Vec < (Span , String) > , }
    };
}

ConditionVisitor!();