macro_rules! GraphQLPlaygroundConfig {
    () => {
        # [doc = " Config for GraphQL Playground"] # [derive (Serialize)] # [serde (rename_all = "camelCase")] pub struct GraphQLPlaygroundConfig < 'a > { endpoint : & 'a str , subscription_endpoint : Option < & 'a str > , headers : Option < HashMap < & 'a str , & 'a str > > , settings : Option < HashMap < & 'a str , Value > > , title : Option < & 'a str > , }
    };
}

GraphQLPlaygroundConfig!();