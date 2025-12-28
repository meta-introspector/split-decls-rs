macro_rules! graphiql_plugin {
    () => {
        # [cfg (feature = "graphiql")] mod graphiql_plugin ;
    };
}

graphiql_plugin!()