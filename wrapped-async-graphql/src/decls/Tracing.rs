macro_rules! deps {
    () => {
        EmptySubscription!();
        Query!();
        EmptyMutation!();
    };
}

macro_rules! Tracing {
    () => {
        deps!();
        # [doc = " Tracing extension"] # [doc = ""] # [doc = " # References"] # [doc = ""] # [doc = " <https://crates.io/crates/tracing>"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use async_graphql::{extensions::Tracing, *};"] # [doc = ""] # [doc = " #[derive(SimpleObject)]"] # [doc = " struct Query {"] # [doc = "     value: i32,"] # [doc = " }"] # [doc = ""] # [doc = " let schema = Schema::build(Query { value: 100 }, EmptyMutation, EmptySubscription)"] # [doc = "     .extension(Tracing)"] # [doc = "     .finish();"] # [doc = ""] # [doc = " # tokio::runtime::Runtime::new().unwrap().block_on(async {"] # [doc = " schema.execute(Request::new(\"{ value }\")).await;"] # [doc = " # });"] # [doc = " ```"] # [cfg_attr (docsrs , doc (cfg (feature = "tracing")))] pub struct Tracing ;
    };
}

Tracing!()