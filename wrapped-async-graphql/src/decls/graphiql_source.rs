macro_rules! graphiql_source {
    () => {
        # [cfg (feature = "graphiql")] mod graphiql_source ;
    };
}

graphiql_source!();