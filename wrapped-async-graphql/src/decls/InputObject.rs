macro_rules! deps {
    () => {
        InputValue!();
        SchemaError!();
        Directive!();
    };
}

macro_rules! InputObject {
    () => {
        deps!();
        # [doc = " A GraphQL input object type"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use async_graphql::{dynamic::*, value, Value};"] # [doc = ""] # [doc = " let my_input = InputObject::new(\"MyInput\")"] # [doc = "     .field(InputValue::new(\"a\", TypeRef::named_nn(TypeRef::INT)))"] # [doc = "     .field(InputValue::new(\"b\", TypeRef::named_nn(TypeRef::INT)));"] # [doc = ""] # [doc = " let query = Object::new(\"Query\").field("] # [doc = "     Field::new(\"add\", TypeRef::named_nn(TypeRef::INT), |ctx| {"] # [doc = "         FieldFuture::new(async move {"] # [doc = "             let input = ctx.args.try_get(\"input\")?;"] # [doc = "             let input = input.object()?;"] # [doc = "             let a = input.try_get(\"a\")?.i64()?;"] # [doc = "             let b = input.try_get(\"b\")?.i64()?;"] # [doc = "             Ok(Some(Value::from(a + b)))"] # [doc = "         })"] # [doc = "     })"] # [doc = "     .argument(InputValue::new(\"input\", TypeRef::named_nn(my_input.type_name())))"] # [doc = " );"] # [doc = ""] # [doc = " # tokio::runtime::Runtime::new().unwrap().block_on(async move {"] # [doc = ""] # [doc = " let schema = Schema::build(query.type_name(), None, None)"] # [doc = "     .register(my_input)"] # [doc = "     .register(query)"] # [doc = "     .finish()?;"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "    schema"] # [doc = "        .execute(\"{ add(input: { a: 10, b: 20 }) }\")"] # [doc = "        .await"] # [doc = "        .into_result()"] # [doc = "        .unwrap()"] # [doc = "        .data,"] # [doc = "    value!({ \"add\": 30 })"] # [doc = " );"] # [doc = ""] # [doc = " # Ok::<_, SchemaError>(())"] # [doc = " # }).unwrap();"] # [doc = " ```"] # [derive (Debug)] pub struct InputObject { pub (crate) name : String , pub (crate) description : Option < String > , pub (crate) fields : IndexMap < String , InputValue > , pub (crate) oneof : bool , inaccessible : bool , tags : Vec < String > , directives : Vec < Directive > , }
    };
}

InputObject!();