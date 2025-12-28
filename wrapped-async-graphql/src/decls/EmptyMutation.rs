macro_rules! deps {
    () => {
        Query!();
        EmptySubscription!();
        Object!();
        Schema!();
    };
}

macro_rules! EmptyMutation {
    () => {
        deps!();
        # [doc = " Empty mutation"] # [doc = ""] # [doc = " Only the parameters used to construct the Schema, representing an"] # [doc = " unconfigured mutation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use async_graphql::*;"] # [doc = ""] # [doc = " struct Query;"] # [doc = ""] # [doc = " #[Object]"] # [doc = " impl Query {"] # [doc = "     async fn value(&self) -> i32 {"] # [doc = "         // A GraphQL Object type must define one or more fields."] # [doc = "         100"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let schema = Schema::new(Query, EmptyMutation, EmptySubscription);"] # [doc = " ```"] # [derive (Default , Copy , Clone)] pub struct EmptyMutation ;
    };
}

EmptyMutation!();