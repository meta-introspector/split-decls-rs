macro_rules! deps {
    () => {
        ScalarValidatorFn!();
        SchemaError!();
        Directive!();
    };
}

macro_rules! Scalar {
    () => {
        deps!();
        # [doc = " A GraphQL scalar type"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use async_graphql::{dynamic::*, value, Value};"] # [doc = ""] # [doc = " let my_scalar = Scalar::new(\"MyScalar\");"] # [doc = ""] # [doc = " let query = Object::new(\"Query\").field(Field::new(\"value\", TypeRef::named_nn(my_scalar.type_name()), |ctx| {"] # [doc = "     FieldFuture::new(async move { Ok(Some(Value::from(\"abc\"))) })"] # [doc = " }));"] # [doc = ""] # [doc = " # tokio::runtime::Runtime::new().unwrap().block_on(async move {"] # [doc = ""] # [doc = " let schema = Schema::build(query.type_name(), None, None)"] # [doc = "     .register(my_scalar)"] # [doc = "     .register(query)"] # [doc = "     .finish()?;"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "    schema"] # [doc = "        .execute(\"{ value }\")"] # [doc = "        .await"] # [doc = "        .into_result()"] # [doc = "        .unwrap()"] # [doc = "        .data,"] # [doc = "    value!({ \"value\": \"abc\" })"] # [doc = " );"] # [doc = ""] # [doc = " # Ok::<_, SchemaError>(())"] # [doc = " # }).unwrap();"] # [doc = " ```"] pub struct Scalar { pub (crate) name : String , pub (crate) description : Option < String > , pub (crate) specified_by_url : Option < String > , pub (crate) validator : Option < ScalarValidatorFn > , inaccessible : bool , tags : Vec < String > , pub (crate) directives : Vec < Directive > , requires_scopes : Vec < String > , }
    };
}

Scalar!();