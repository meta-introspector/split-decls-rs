macro_rules! deps {
    () => {
        TraitEnvironment!();
    };
}

macro_rules! AutoderefCtx {
    () => {
        deps!();
        pub (crate) trait AutoderefCtx < 'db > { fn infcx (& self) -> & InferCtxt < 'db > ; fn env (& self) -> & TraitEnvironment < 'db > ; }
    };
}

AutoderefCtx!()