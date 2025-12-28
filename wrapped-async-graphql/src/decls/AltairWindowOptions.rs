macro_rules! deps {
    () => {
        AltairHttpVerb!();
        AltairAuthorizationProviderInput!();
    };
}

macro_rules! AltairWindowOptions {
    () => {
        deps!();
        # [doc = " Altair window [options](https://github.com/altair-graphql/altair/blob/master/packages/altair-core/src/config.ts#L10)"] # [derive (Default , Serialize , Deserialize , JsonSchema)] # [serde (rename_all = "camelCase")] pub struct AltairWindowOptions { # [doc = " Initial name of the window"] # [serde (default , skip_serializing_if = "Option::is_none")] pub initial_name : Option < String > , # [doc = " URL to set as the server endpoint"] # [serde (rename = "endpointURL" , default , skip_serializing_if = "Option::is_none")] pub endpoint_url : Option < String > , # [doc = " URL to set as the subscription endpoint. This can be relative or"] # [doc = " absolute."] # [serde (default , skip_serializing_if = "Option::is_none")] pub subscriptions_endpoint : Option < String > , # [doc = " URL protocol for the subscription endpoint. This is used if the"] # [doc = " specified subscriptions endpoint is relative."] # [doc = ""] # [doc = " e.g. wss"] # [serde (default , skip_serializing_if = "Option::is_none")] pub subscriptions_protocol : Option < String > , # [doc = " Initial query to be added"] # [serde (default , skip_serializing_if = "Option::is_none")] pub initial_query : Option < String > , # [doc = " Initial variables to be added"] # [serde (default , skip_serializing_if = "Option::is_none")] pub initial_variables : Option < String > , # [doc = " Initial pre-request script to be added"] # [serde (default , skip_serializing_if = "Option::is_none")] pub initial_pre_request_script : Option < String > , # [doc = " Initial post-request script to be added"] # [serde (default , skip_serializing_if = "Option::is_none")] pub initial_post_request_script : Option < String > , # [doc = " Initial authorization type and data"] # [serde (default , skip_serializing_if = "Option::is_none")] pub initial_authorization : Option < AltairAuthorizationProviderInput > , # [doc = " Initial headers object to be added"] # [doc = " ```js"] # [doc = " {"] # [doc = "  'X-GraphQL-Token': 'asd7-237s-2bdk-nsdk4'"] # [doc = " }"] # [doc = " ```"] # [serde (default , skip_serializing_if = "HashMap::is_empty")] pub initial_headers : HashMap < String , String > , # [doc = " Initial subscriptions connection params"] # [serde (default , skip_serializing_if = "HashMap::is_empty")] pub initial_subscriptions_payload : HashMap < String , String > , # [doc = " HTTP method to use for making requests"] # [serde (default , skip_serializing_if = "Option::is_none")] pub initial_http_method : Option < AltairHttpVerb > , }
    };
}

AltairWindowOptions!();