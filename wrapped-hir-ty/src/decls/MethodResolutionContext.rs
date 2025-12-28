macro_rules! deps {
    () => {
        MethodResolutionUnstableFeatures!();
        TraitEnvironment!();
    };
}

macro_rules! MethodResolutionContext {
    () => {
        deps!();
        pub struct MethodResolutionContext < 'a , 'db > { pub infcx : & 'a InferCtxt < 'db > , pub resolver : & 'a Resolver < 'db > , pub env : & 'a TraitEnvironment < 'db > , pub traits_in_scope : & 'a FxHashSet < TraitId > , pub edition : Edition , pub unstable_features : & 'a MethodResolutionUnstableFeatures , }
    };
}

MethodResolutionContext!()