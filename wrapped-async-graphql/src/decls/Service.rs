macro_rules! Service {
    () => {
        # [doc = " Federation service"] # [derive (SimpleObject)] # [graphql (internal , name = "_Service")] struct Service { sdl : Option < String > , }
    };
}

Service!()