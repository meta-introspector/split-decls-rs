macro_rules! deps {
    () => {
        TypeTrace!();
        PredicateObligations!();
        DefineOpaqueTypes!();
        InferCtxt!();
    };
}

macro_rules! TypeRelating {
    () => {
        deps!();
        # [doc = " Enforce that `a` is equal to or a subtype of `b`."] pub (crate) struct TypeRelating < 'infcx , 'tcx > { infcx : & 'infcx InferCtxt < 'tcx > , trace : TypeTrace < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , define_opaque_types : DefineOpaqueTypes , ambient_variance : ty :: Variance , obligations : PredicateObligations < 'tcx > , # [doc = " The cache only tracks the `ambient_variance` as it's the"] # [doc = " only field which is mutable and which meaningfully changes"] # [doc = " the result when relating types."] # [doc = ""] # [doc = " The cache does not track whether the state of the"] # [doc = " `InferCtxt` has been changed or whether we've added any"] # [doc = " obligations to `self.goals`. Whether a goal is added"] # [doc = " once or multiple times is not really meaningful."] # [doc = ""] # [doc = " Changes in the inference state may delay some type inference to"] # [doc = " the next fulfillment loop. Given that this loop is already"] # [doc = " necessary, this is also not a meaningful change. Consider"] # [doc = " the following three relations:"] # [doc = " ```text"] # [doc = " Vec<?0> sub Vec<?1>"] # [doc = " ?0 eq u32"] # [doc = " Vec<?0> sub Vec<?1>"] # [doc = " ```"] # [doc = " Without a cache, the second `Vec<?0> sub Vec<?1>` would eagerly"] # [doc = " constrain `?1` to `u32`. When using the cache entry from the"] # [doc = " first time we've related these types, this only happens when"] # [doc = " later proving the `Subtype(?0, ?1)` goal from the first relation."] cache : DelayedSet < (ty :: Variance , Ty < 'tcx > , Ty < 'tcx >) > , }
    };
}

TypeRelating!();