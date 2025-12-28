macro_rules! deps {
    () => {
        GraphiQLSource!();
        Credentials!();
        GraphiQLPlugin!();
        GraphiQLVersion!();
    };
}

macro_rules! impl_620 {
    () => {
        deps!();
        impl < 'a > GraphiQLSource < 'a > { # [doc = " Creates a builder for constructing a GraphiQL (v2) HTML page."] pub fn build () -> GraphiQLSource < 'a > { Default :: default () } # [doc = " Sets the endpoint of the server GraphiQL will connect to."] # [must_use] pub fn endpoint (self , endpoint : & 'a str) -> GraphiQLSource < 'a > { GraphiQLSource { endpoint , .. self } } # [doc = " Sets the subscription endpoint of the server GraphiQL will connect to."] pub fn subscription_endpoint (self , endpoint : & 'a str) -> GraphiQLSource < 'a > { GraphiQLSource { subscription_endpoint : Some (endpoint) , .. self } } # [doc = " Sets a header to be sent with requests GraphiQL will send."] pub fn header (self , name : & 'a str , value : & 'a str) -> GraphiQLSource < 'a > { let mut headers = self . headers . unwrap_or_default () ; headers . insert (name , value) ; GraphiQLSource { headers : Some (headers) , .. self } } # [doc = " Sets the version of GraphiQL to be fetched."] pub fn version (self , value : & 'a str) -> GraphiQLSource < 'a > { GraphiQLSource { version : GraphiQLVersion (value) , .. self } } # [doc = " Sets a WS connection param to be sent during GraphiQL WS connections."] pub fn ws_connection_param (self , name : & 'a str , value : & 'a str) -> GraphiQLSource < 'a > { let mut ws_connection_params = self . ws_connection_params . unwrap_or_default () ; ws_connection_params . insert (name , value) ; GraphiQLSource { ws_connection_params : Some (ws_connection_params) , .. self } } # [doc = " Sets the html document title."] pub fn title (self , title : & 'a str) -> GraphiQLSource < 'a > { GraphiQLSource { title : Some (title) , .. self } } # [doc = " Sets credentials option for the fetch requests."] pub fn credentials (self , credentials : Credentials) -> GraphiQLSource < 'a > { GraphiQLSource { credentials , .. self } } # [doc = " Sets plugins"] pub fn plugins (self , plugins : & 'a [GraphiQLPlugin]) -> GraphiQLSource < 'a > { GraphiQLSource { plugins , .. self } } # [doc = " Returns a GraphiQL (v2) HTML page."] pub fn finish (self) -> String { let mut handlebars = Handlebars :: new () ; handlebars . register_template_string ("graphiql_v2_source" , include_str ! ("./graphiql_v2_source.hbs") ,) . expect ("Failed to register template") ; handlebars . render ("graphiql_v2_source" , & self) . expect ("Failed to render template") } }
    };
}

impl_620!()