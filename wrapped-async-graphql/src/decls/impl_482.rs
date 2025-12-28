macro_rules! deps {
    () => {
        FieldFuture!();
        Result!();
        Scalar!();
        Interface!();
        ValidationMode!();
        SchemaInner!();
        Object!();
        ExtensionFactory!();
        IntrospectionMode!();
        ResolverContext!();
        Union!();
        ID!();
        SchemaEnv!();
        SchemaEnvInner!();
        TypeRef!();
        Schema!();
        Query!();
        Registry!();
        Any!();
        SchemaError!();
        SchemaBuilder!();
        Upload!();
    };
}

macro_rules! impl_482 {
    () => {
        deps!();
        impl SchemaBuilder { # [doc = " Register a GraphQL type"] # [must_use] pub fn register (mut self , ty : impl Into < Type >) -> Self { let ty = ty . into () ; self . types . insert (ty . name () . to_string () , ty) ; self } # [doc = " Enable uploading files (register Upload type)."] pub fn enable_uploading (mut self) -> Self { self . types . insert (TypeRef :: UPLOAD . to_string () , Type :: Upload) ; self } # [doc = " Add a global data that can be accessed in the `Schema`. You access it"] # [doc = " with `Context::data`."] # [must_use] pub fn data < D : Any + Send + Sync > (mut self , data : D) -> Self { self . data . insert (data) ; self } # [doc = " Add an extension to the schema."] # [must_use] pub fn extension (mut self , extension : impl ExtensionFactory) -> Self { self . extensions . push (Box :: new (extension)) ; self } # [doc = " Set the maximum complexity a query can have. By default, there is no"] # [doc = " limit."] # [must_use] pub fn limit_complexity (mut self , complexity : usize) -> Self { self . complexity = Some (complexity) ; self } # [doc = " Set the maximum depth a query can have. By default, there is no limit."] # [must_use] pub fn limit_depth (mut self , depth : usize) -> Self { self . depth = Some (depth) ; self } # [doc = " Set the maximum recursive depth a query can have. (default: 32)"] # [doc = ""] # [doc = " If the value is too large, stack overflow may occur, usually `32` is"] # [doc = " enough."] # [must_use] pub fn limit_recursive_depth (mut self , depth : usize) -> Self { self . recursive_depth = depth ; self } # [doc = " Set the maximum number of directives on a single field. (default: no"] # [doc = " limit)"] pub fn limit_directives (mut self , max_directives : usize) -> Self { self . max_directives = Some (max_directives) ; self } # [doc = " Set the validation mode, default is `ValidationMode::Strict`."] # [must_use] pub fn validation_mode (mut self , validation_mode : ValidationMode) -> Self { self . validation_mode = validation_mode ; self } # [doc = " Disable field suggestions."] # [must_use] pub fn disable_suggestions (mut self) -> Self { self . enable_suggestions = false ; self } # [doc = " Disable introspection queries."] # [must_use] pub fn disable_introspection (mut self) -> Self { self . introspection_mode = IntrospectionMode :: Disabled ; self } # [doc = " Only process introspection queries, everything else is processed as an"] # [doc = " error."] # [must_use] pub fn introspection_only (mut self) -> Self { self . introspection_mode = IntrospectionMode :: IntrospectionOnly ; self } # [doc = " Enable federation, which is automatically enabled if the Query has least"] # [doc = " one entity definition."] # [must_use] pub fn enable_federation (mut self) -> Self { self . enable_federation = true ; self } # [doc = " Set the entity resolver for federation"] pub fn entity_resolver < F > (self , resolver_fn : F) -> Self where F : for < 'a > Fn (ResolverContext < 'a >) -> FieldFuture < 'a > + Send + Sync + 'static , { Self { entity_resolver : Some (Box :: new (resolver_fn)) , .. self } } # [doc = " Consumes this builder and returns a schema."] pub fn finish (mut self) -> Result < Schema , SchemaError > { let mut registry = Registry { types : Default :: default () , directives : Default :: default () , implements : Default :: default () , query_type : self . query_type , mutation_type : self . mutation_type , subscription_type : self . subscription_type , introspection_mode : self . introspection_mode , enable_federation : false , federation_subscription : false , ignore_name_conflicts : Default :: default () , enable_suggestions : self . enable_suggestions , } ; registry . add_system_types () ; for ty in self . types . values () { ty . register (& mut registry) ? ; } update_interface_possible_types (& mut self . types , & mut registry) ; for ty in ["Int" , "Float" , "Boolean" , "String" , "ID"] { self . types . insert (ty . to_string () , Type :: Scalar (Scalar :: new (ty))) ; } if matches ! (self . introspection_mode , IntrospectionMode :: Enabled | IntrospectionMode :: IntrospectionOnly) { registry . create_introspection_types () ; } if self . enable_federation || registry . has_entities () { registry . enable_federation = true ; registry . create_federation_types () ; let entity = self . types . values () . filter (| ty | match ty { Type :: Object (obj) => obj . is_entity () , Type :: Interface (interface) => interface . is_entity () , _ => false , }) . fold (Union :: new ("_Entity") , | entity , ty | { entity . possible_type (ty . name ()) }) ; self . types . insert ("_Entity" . to_string () , Type :: Union (entity)) ; } let inner = SchemaInner { env : SchemaEnv (Arc :: new (SchemaEnvInner { registry , data : self . data , custom_directives : Default :: default () , })) , extensions : self . extensions , types : self . types , recursive_depth : self . recursive_depth , max_directives : self . max_directives , complexity : self . complexity , depth : self . depth , validation_mode : self . validation_mode , entity_resolver : self . entity_resolver , } ; inner . check () ? ; Ok (Schema (Arc :: new (inner))) } }
    };
}

impl_482!();