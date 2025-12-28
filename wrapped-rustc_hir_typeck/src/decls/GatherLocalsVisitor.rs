macro_rules! deps {
    () => {
        FnCtxt!();
    };
}

macro_rules! GatherLocalsVisitor {
    () => {
        deps!();
        # [doc = " The `GatherLocalsVisitor` is responsible for initializing local variable types"] # [doc = " in the [`ty::TypeckResults`] for all subpatterns in statements and expressions"] # [doc = " like `let`, `match`, and params of function bodies. It also adds `Sized` bounds"] # [doc = " for these types (with exceptions for unsized feature gates like `unsized_fn_params`)."] # [doc = ""] # [doc = " Failure to visit locals will cause an ICE in writeback when the local's type is"] # [doc = " resolved. Visiting locals twice will ICE in the `GatherLocalsVisitor`, since it"] # [doc = " will overwrite the type previously stored in the local."] pub (super) struct GatherLocalsVisitor < 'a , 'tcx > { fcx : & 'a FnCtxt < 'a , 'tcx > , outermost_fn_param_pat : Option < (Span , HirId) > , }
    };
}

GatherLocalsVisitor!();