macro_rules! deps {
    () => {
        EmptyMutation!();
        ContextBase!();
        OneofObjectType!();
        EmptySubscription!();
        SelectionField!();
        Object!();
        Query!();
        Field!();
        Context!();
        ServerResult!();
        InputType!();
        Lookahead!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl < 'a > ContextBase < 'a , & 'a Positioned < Field > > { # [doc (hidden)] pub fn param_value < T : InputType > (& self , name : & str , default : Option < fn () -> T > ,) -> ServerResult < (Pos , T) > { self . get_param_value (& self . item . node . arguments , name , default) } # [doc (hidden)] pub fn oneof_param_value < T : OneofObjectType > (& self) -> ServerResult < (Pos , T) > { use indexmap :: IndexMap ; let mut map = IndexMap :: new () ; for (name , value) in & self . item . node . arguments { let value = self . resolve_input_value (value . clone ()) ? ; map . insert (name . node . clone () , value) ; } InputType :: parse (Some (Value :: Object (map))) . map (| value | (self . item . pos , value)) . map_err (| e | e . into_server_error (self . item . pos)) } # [doc = " Creates a uniform interface to inspect the forthcoming selections."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use async_graphql::*;"] # [doc = ""] # [doc = " #[derive(SimpleObject)]"] # [doc = " struct Detail {"] # [doc = "     c: i32,"] # [doc = "     d: i32,"] # [doc = " }"] # [doc = ""] # [doc = " #[derive(SimpleObject)]"] # [doc = " struct MyObj {"] # [doc = "     a: i32,"] # [doc = "     b: i32,"] # [doc = "     detail: Detail,"] # [doc = " }"] # [doc = ""] # [doc = " struct Query;"] # [doc = ""] # [doc = " #[Object]"] # [doc = " impl Query {"] # [doc = "     async fn obj(&self, ctx: &Context<'_>) -> MyObj {"] # [doc = "         if ctx.look_ahead().field(\"a\").exists() {"] # [doc = "             // This is a query like `obj { a }`"] # [doc = "         } else if ctx.look_ahead().field(\"detail\").field(\"c\").exists() {"] # [doc = "             // This is a query like `obj { detail { c } }`"] # [doc = "         } else {"] # [doc = "             // This query doesn't have `a`"] # [doc = "         }"] # [doc = "         unimplemented!()"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub fn look_ahead (& self) -> Lookahead { Lookahead :: new (& self . query_env . fragments , & self . item . node , self) } # [doc = " Get the current field."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use async_graphql::*;"] # [doc = ""] # [doc = " #[derive(SimpleObject)]"] # [doc = " struct MyObj {"] # [doc = "     a: i32,"] # [doc = "     b: i32,"] # [doc = "     c: i32,"] # [doc = " }"] # [doc = ""] # [doc = " pub struct Query;"] # [doc = ""] # [doc = " #[Object]"] # [doc = " impl Query {"] # [doc = "     async fn obj(&self, ctx: &Context<'_>) -> MyObj {"] # [doc = "         let fields = ctx"] # [doc = "             .field()"] # [doc = "             .selection_set()"] # [doc = "             .map(|field| field.name())"] # [doc = "             .collect::<Vec<_>>();"] # [doc = "         assert_eq!(fields, vec![\"a\", \"b\", \"c\"]);"] # [doc = "         MyObj { a: 1, b: 2, c: 3 }"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " # tokio::runtime::Runtime::new().unwrap().block_on(async move {"] # [doc = " let schema = Schema::new(Query, EmptyMutation, EmptySubscription);"] # [doc = " assert!(schema.execute(\"{ obj { a b c }}\").await.is_ok());"] # [doc = " assert!(schema.execute(\"{ obj { a ... { b c } }}\").await.is_ok());"] # [doc = " assert!("] # [doc = "     schema"] # [doc = "         .execute(\"{ obj { a ... BC }} fragment BC on MyObj { b c }\")"] # [doc = "         .await"] # [doc = "         .is_ok()"] # [doc = " );"] # [doc = " # });"] # [doc = " ```"] pub fn field (& self) -> SelectionField { SelectionField { fragments : & self . query_env . fragments , field : & self . item . node , context : self , } } }
    };
}

impl_352!();