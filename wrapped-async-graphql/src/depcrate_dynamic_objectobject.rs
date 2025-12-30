// Generated macro for Object (struct)
macro_rules! Depcrate_dynamic_objectObject {
() => {
// Module: crate::dynamic::object
// Provides: {"Object"}
// Dependencies: {}
# [doc = " A GraphQL object type"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use async_graphql::{dynamic::*, value, Value};"] # [doc = ""] # [doc = " let query = Object::new(\"Query\").field(Field::new(\"value\", TypeRef::named_nn(TypeRef::STRING), |ctx| {"] # [doc = "     FieldFuture::new(async move { Ok(Some(Value::from(\"abc\"))) })"] # [doc = " }));"] # [doc = ""] # [doc = " # tokio::runtime::Runtime::new().unwrap().block_on(async move {"] # [doc = ""] # [doc = " let schema = Schema::build(query.type_name(), None, None)"] # [doc = "     .register(query)"] # [doc = "     .finish()?;"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "    schema"] # [doc = "        .execute(\"{ value }\")"] # [doc = "        .await"] # [doc = "        .into_result()"] # [doc = "        .unwrap()"] # [doc = "        .data,"] # [doc = "    value!({ \"value\": \"abc\" })"] # [doc = " );"] # [doc = ""] # [doc = " # Ok::<_, SchemaError>(())"] # [doc = " # }).unwrap();"] # [doc = " ```"] # [derive (Debug)] pub struct Object { pub (crate) name : String , pub (crate) description : Option < String > , pub (crate) fields : IndexMap < String , Field > , pub (crate) implements : IndexSet < String > , keys : Vec < String > , extends : bool , shareable : bool , resolvable : bool , inaccessible : bool , interface_object : bool , tags : Vec < String > , pub (crate) directives : Vec < Directive > , requires_scopes : Vec < String > , }
};
}
