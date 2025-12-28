macro_rules! deps {
    () => {
        Query!();
        EmptySubscription!();
        Scope!();
        EmptyMutation!();
    };
}

macro_rules! CacheControl {
    () => {
        deps!();
        # [doc = " Cache control value"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use async_graphql::*;"] # [doc = ""] # [doc = " struct Query;"] # [doc = ""] # [doc = " #[Object(cache_control(max_age = 60))]"] # [doc = " impl Query {"] # [doc = "     #[graphql(cache_control(max_age = 30))]"] # [doc = "     async fn value1(&self) -> i32 {"] # [doc = "         0"] # [doc = "     }"] # [doc = ""] # [doc = "     #[graphql(cache_control(private))]"] # [doc = "     async fn value2(&self) -> i32 {"] # [doc = "         0"] # [doc = "     }"] # [doc = ""] # [doc = "     #[graphql(cache_control(no_cache))]"] # [doc = "     async fn value3(&self) -> i32 {"] # [doc = "         0"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " # tokio::runtime::Runtime::new().unwrap().block_on(async {"] # [doc = " let schema = Schema::new(Query, EmptyMutation, EmptySubscription);"] # [doc = " assert_eq!("] # [doc = "     schema"] # [doc = "         .execute(\"{ value1 }\")"] # [doc = "         .await"] # [doc = "         .into_result()"] # [doc = "         .unwrap()"] # [doc = "         .cache_control,"] # [doc = "     CacheControl {"] # [doc = "         public: true,"] # [doc = "         max_age: 30"] # [doc = "     }"] # [doc = " );"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     schema"] # [doc = "         .execute(\"{ value2 }\")"] # [doc = "         .await"] # [doc = "         .into_result()"] # [doc = "         .unwrap()"] # [doc = "         .cache_control,"] # [doc = "     CacheControl {"] # [doc = "         public: false,"] # [doc = "         max_age: 60"] # [doc = "     }"] # [doc = " );"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     schema"] # [doc = "         .execute(\"{ value1 value2 }\")"] # [doc = "         .await"] # [doc = "         .into_result()"] # [doc = "         .unwrap()"] # [doc = "         .cache_control,"] # [doc = "     CacheControl {"] # [doc = "         public: false,"] # [doc = "         max_age: 30"] # [doc = "     }"] # [doc = " );"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     schema"] # [doc = "         .execute(\"{ value1 value2 value3 }\")"] # [doc = "         .await"] # [doc = "         .into_result()"] # [doc = "         .unwrap()"] # [doc = "         .cache_control,"] # [doc = "     CacheControl {"] # [doc = "         public: false,"] # [doc = "         max_age: -1"] # [doc = "     }"] # [doc = " );"] # [doc = " # });"] # [doc = " ```"] # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub struct CacheControl { # [doc = " Scope is public, default is true."] pub public : bool , # [doc = " Cache max age, `-1` represent `no-cache`, default is 0."] pub max_age : i32 , }
    };
}

CacheControl!()