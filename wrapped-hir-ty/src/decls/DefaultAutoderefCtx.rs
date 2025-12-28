macro_rules! deps {
    () => {
        TraitEnvironment!();
    };
}

macro_rules! DefaultAutoderefCtx {
    () => {
        deps!();
        pub (crate) struct DefaultAutoderefCtx < 'a , 'db > { infcx : & 'a InferCtxt < 'db > , env : & 'a TraitEnvironment < 'db > , }
    };
}

DefaultAutoderefCtx!();