macro_rules! deps {
    () => {
        Object!();
        Query!();
        EmptyMutation!();
        EmptySubscription!();
    };
}

macro_rules! MaybeUndefined {
    () => {
        deps!();
        # [doc = " Similar to `Option`, but it has three states, `undefined`, `null` and `x`."] # [doc = ""] # [doc = " **Reference:** <https://spec.graphql.org/October2021/#sec-Null-Value>"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use async_graphql::*;"] # [doc = ""] # [doc = " struct Query;"] # [doc = ""] # [doc = " #[Object]"] # [doc = " impl Query {"] # [doc = "     async fn value1(&self, input: MaybeUndefined<i32>) -> i32 {"] # [doc = "         if input.is_null() {"] # [doc = "             1"] # [doc = "         } else if input.is_undefined() {"] # [doc = "             2"] # [doc = "         } else {"] # [doc = "             input.take().unwrap()"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " # tokio::runtime::Runtime::new().unwrap().block_on(async {"] # [doc = " let schema = Schema::new(Query, EmptyMutation, EmptySubscription);"] # [doc = " let query = r#\""] # [doc = "     {"] # [doc = "         v1:value1(input: 99)"] # [doc = "         v2:value1(input: null)"] # [doc = "         v3:value1"] # [doc = "     }\"#;"] # [doc = " assert_eq!("] # [doc = "     schema.execute(query).await.into_result().unwrap().data,"] # [doc = "     value!({"] # [doc = "         \"v1\": 99,"] # [doc = "         \"v2\": 1,"] # [doc = "         \"v3\": 2,"] # [doc = "     })"] # [doc = " );"] # [doc = " # });"] # [doc = " ```"] # [allow (missing_docs)] # [derive (Copy , Clone , PartialEq , PartialOrd , Eq , Ord , Debug , Hash)] pub enum MaybeUndefined < T > { Undefined , Null , Value (T) , }
    };
}

MaybeUndefined!()