macro_rules! FunctionalVariances {
    () => {
        # [doc = " Computes the variances of regions that appear in the type, but considering"] # [doc = " late-bound regions too, which don't have their variance computed usually."] # [doc = ""] # [doc = " Like generalization, this is a unary operation implemented on top of the binary"] # [doc = " relation infrastructure, mostly because it's much easier to have the relation"] # [doc = " track the variance for you, rather than having to do it yourself."] struct FunctionalVariances < 'tcx > { tcx : TyCtxt < 'tcx > , variances : FxHashMap < DefId , ty :: Variance > , ambient_variance : ty :: Variance , generics : & 'tcx ty :: Generics , }
    };
}

FunctionalVariances!();