macro_rules! deps {
    () => {
        GraphQLPlaygroundConfig!();
    };
}

macro_rules! impl_640 {
    () => {
        deps!();
        impl < 'a > GraphQLPlaygroundConfig < 'a > { # [doc = " Create a config for GraphQL playground."] pub fn new (endpoint : & 'a str) -> Self { Self { endpoint , subscription_endpoint : None , headers : Default :: default () , settings : Default :: default () , title : Default :: default () , } } # [doc = " Set subscription endpoint, for example: `ws://localhost:8000`."] # [must_use] pub fn subscription_endpoint (mut self , endpoint : & 'a str) -> Self { self . subscription_endpoint = Some (endpoint) ; self } # [doc = " Set HTTP header for per query."] # [must_use] pub fn with_header (mut self , name : & 'a str , value : & 'a str) -> Self { if let Some (headers) = & mut self . headers { headers . insert (name , value) ; } else { let mut headers = HashMap :: new () ; headers . insert (name , value) ; self . headers = Some (headers) ; } self } # [doc = " Set the html document title."] # [must_use] pub fn title (mut self , title : & 'a str) -> Self { self . title = Some (title) ; self } # [doc = " Set Playground setting for per query."] # [doc = ""] # [doc = " ```"] # [doc = " # use async_graphql::Value;"] # [doc = " # use async_graphql::http::GraphQLPlaygroundConfig;"] # [doc = " GraphQLPlaygroundConfig::new(\"/api/graphql\")"] # [doc = "     .with_setting(\"setting\", false)"] # [doc = "     .with_setting(\"other\", Value::Null);"] # [doc = " ```"] # [must_use] pub fn with_setting (mut self , name : & 'a str , value : impl Into < Value >) -> Self { let value = value . into () ; if let Some (settings) = & mut self . settings { settings . insert (name , value) ; } else { let mut settings = HashMap :: new () ; settings . insert (name , value) ; self . settings = Some (settings) ; } self } }
    };
}

impl_640!()