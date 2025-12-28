macro_rules! FreeRegionsVisitor {
    () => {
        # [doc = " Visits free regions in the type that are relevant for liveness computation."] # [doc = " These regions are passed to `OP`."] # [doc = ""] # [doc = " Specifically, we visit all of the regions of types recursively, except if"] # [doc = " the type is an alias, we look at the outlives bounds in the param-env"] # [doc = " and alias's item bounds. If there is a unique outlives bound, then visit"] # [doc = " that instead. If there is not a unique but there is a `'static` outlives"] # [doc = " bound, then don't visit anything. Otherwise, walk through the opaque's"] # [doc = " regions structurally."] pub struct FreeRegionsVisitor < 'tcx , OP : FnMut (ty :: Region < 'tcx >) > { pub tcx : TyCtxt < 'tcx > , pub param_env : ty :: ParamEnv < 'tcx > , pub op : OP , }
    };
}

FreeRegionsVisitor!()