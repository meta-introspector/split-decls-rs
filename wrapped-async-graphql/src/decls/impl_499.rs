macro_rules! deps {
    () => {
        QueryPathSegment!();
        MetaField!();
        Subscription!();
        BoxFieldStream!();
        Schema!();
        Object!();
        Response!();
        MetaType!();
        ContextSelectionSet!();
        ServerResult!();
        SchemaError!();
        Result!();
        ResolveInfo!();
        ResolverContext!();
        Data!();
        QueryPathNode!();
        SubscriptionField!();
        Field!();
        Registry!();
        ObjectAccessor!();
        FieldValue!();
    };
}

macro_rules! impl_499 {
    () => {
        deps!();
        impl Subscription { # [doc = " Create a GraphQL object type"] # [inline] pub fn new (name : impl Into < String >) -> Self { Self { name : name . into () , description : None , fields : Default :: default () , } } impl_set_description ! () ; # [doc = " Add an field to the object"] # [inline] pub fn field (mut self , field : SubscriptionField) -> Self { assert ! (! self . fields . contains_key (& field . name) , "Field `{}` already exists" , field . name) ; self . fields . insert (field . name . clone () , field) ; self } # [doc = " Returns the type name"] # [inline] pub fn type_name (& self) -> & str { & self . name } pub (crate) fn register (& self , registry : & mut Registry) -> Result < () , SchemaError > { let mut fields = IndexMap :: new () ; for field in self . fields . values () { let mut args = IndexMap :: new () ; for argument in field . arguments . values () { args . insert (argument . name . clone () , argument . to_meta_input_value ()) ; } fields . insert (field . name . clone () , MetaField { name : field . name . clone () , description : field . description . clone () , args , ty : field . ty . to_string () , deprecation : field . deprecation . clone () , cache_control : Default :: default () , external : false , requires : None , provides : None , visible : None , shareable : false , inaccessible : false , tags : vec ! [] , override_from : None , compute_complexity : None , directive_invocations : vec ! [] , requires_scopes : vec ! [] , } ,) ; } registry . types . insert (self . name . clone () , MetaType :: Object { name : self . name . clone () , description : self . description . clone () , fields , cache_control : Default :: default () , extends : false , shareable : false , resolvable : true , keys : None , visible : None , inaccessible : false , interface_object : false , tags : vec ! [] , is_subscription : true , rust_typename : None , directive_invocations : vec ! [] , requires_scopes : vec ! [] , } ,) ; Ok (()) } pub (crate) fn collect_streams < 'a > (& self , schema : & Schema , ctx : & ContextSelectionSet < 'a > , streams : & mut Vec < BoxFieldStream < 'a > > , root_value : & 'a FieldValue < 'static > ,) { for selection in & ctx . item . node . items { if let Selection :: Field (field) = & selection . node { if let Some (field_def) = self . fields . get (field . node . name . node . as_str ()) { let schema = schema . clone () ; let field_type = field_def . ty . clone () ; let resolver_fn = field_def . resolver_fn . clone () ; let ctx = ctx . clone () ; streams . push (async_stream :: try_stream ! { let ctx_field = ctx . with_field (field) ; let field_name = ctx_field . item . node . response_key () . node . clone () ; let arguments = ObjectAccessor (Cow :: Owned (field . node . arguments . iter () . map (| (name , value) | { ctx_field . resolve_input_value (value . clone ()) . map (| value | (name . node . clone () , value)) }) . collect ::< ServerResult < IndexMap < Name , Value >>> () ?,)) ; let mut stream = resolver_fn (ResolverContext { ctx : & ctx_field , args : arguments , parent_value : root_value , }) . 0 . await . map_err (| err | ctx_field . set_error_path (err . into_server_error (ctx_field . item . pos))) ?; while let Some (value) = stream . next () . await . transpose () . map_err (| err | ctx_field . set_error_path (err . into_server_error (ctx_field . item . pos))) ? { let f = | execute_data : Option < Data >| { let schema = schema . clone () ; let field_name = field_name . clone () ; let field_type = field_type . clone () ; let ctx_field = ctx_field . clone () ; async move { let mut ctx_field = ctx_field . clone () ; ctx_field . execute_data = execute_data . as_ref () ; let ri = ResolveInfo { path_node : & QueryPathNode { parent : None , segment : QueryPathSegment :: Name (& field_name) , } , parent_type : schema . 0 . env . registry . subscription_type . as_ref () . unwrap () , return_type : & field_type . to_string () , name : field . node . name . node . as_str () , alias : field . node . alias . as_ref () . map (| alias | alias . node . as_str ()) , is_for_introspection : false , field : & field . node , } ; let resolve_fut = resolve (& schema , & ctx_field , & field_type , Some (& value)) ; futures_util :: pin_mut ! (resolve_fut) ; let value = ctx_field . query_env . extensions . resolve (ri , & mut resolve_fut) . await ; match value { Ok (value) => { let mut map = IndexMap :: new () ; map . insert (field_name . clone () , value . unwrap_or_default ()) ; Response :: new (Value :: Object (map)) } , Err (err) => Response :: from_errors (vec ! [err]) , } } } ; let resp = ctx_field . query_env . extensions . execute (ctx_field . query_env . operation_name . as_deref () , f) . await ; let is_err = ! resp . errors . is_empty () ; yield resp ; if is_err { break ; } } } . map (| res | { res . unwrap_or_else (| err | Response :: from_errors (vec ! [err])) }) . boxed () ,) ; } } } } }
    };
}

impl_499!()