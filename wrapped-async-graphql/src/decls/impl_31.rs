macro_rules! deps {
    () => {
        EmptyMutation!();
        PathSegment!();
        EmptySubscription!();
        Any!();
        Query!();
        Object!();
        ServerError!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl ServerError { # [doc = " Create a new server error with the message."] pub fn new (message : impl Into < String > , pos : Option < Pos >) -> Self { Self { message : message . into () , source : None , locations : pos . map (| pos | vec ! [pos]) . unwrap_or_default () , path : Vec :: new () , extensions : None , } } # [doc = " Get the source of the error."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::io::ErrorKind;"] # [doc = ""] # [doc = " use async_graphql::*;"] # [doc = ""] # [doc = " struct Query;"] # [doc = ""] # [doc = " #[Object]"] # [doc = " impl Query {"] # [doc = "     async fn value(&self) -> Result<i32> {"] # [doc = "         Err(Error::new_with_source(std::io::Error::other(\"my error\")))"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let schema = Schema::new(Query, EmptyMutation, EmptySubscription);"] # [doc = ""] # [doc = " # tokio::runtime::Runtime::new().unwrap().block_on(async move {"] # [doc = " let err = schema"] # [doc = "     .execute(\"{ value }\")"] # [doc = "     .await"] # [doc = "     .into_result()"] # [doc = "     .unwrap_err()"] # [doc = "     .remove(0);"] # [doc = " assert!(err.source::<std::io::Error>().is_some());"] # [doc = " # });"] # [doc = " ```"] pub fn source < T : Any + Send + Sync > (& self) -> Option < & T > { self . source . as_ref () . map (| err | err . downcast_ref ()) . flatten () } # [doc (hidden)] # [must_use] pub fn with_path (self , path : Vec < PathSegment >) -> Self { Self { path , .. self } } }
    };
}

impl_31!()