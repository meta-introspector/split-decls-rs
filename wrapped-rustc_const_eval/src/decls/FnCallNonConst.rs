macro_rules! FnCallNonConst {
    () => {
        # [doc = " A function call where the callee is not marked as `const`."] # [derive (Debug , Clone , Copy)] pub (crate) struct FnCallNonConst < 'tcx > { pub callee : DefId , pub args : GenericArgsRef < 'tcx > , pub span : Span , pub call_source : CallSource , }
    };
}

FnCallNonConst!();