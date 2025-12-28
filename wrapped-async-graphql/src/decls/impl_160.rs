macro_rules! deps {
    () => {
        ID!();
        Object!();
        FurColor!();
        ComplicatedArgs!();
        ComplexInput!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        # [Object (internal)] impl ComplicatedArgs { async fn int_arg_field (& self , int_arg : Option < i32 >) -> Option < String > { unimplemented ! () } async fn non_null_int_arg_field (& self , non_null_int_arg : i32) -> Option < String > { unimplemented ! () } async fn string_arg_field (& self , string_arg : Option < String >) -> Option < String > { unimplemented ! () } async fn boolean_arg_field (& self , boolean_arg : Option < bool >) -> Option < String > { unimplemented ! () } async fn enum_arg_field (& self , enum_arg : Option < FurColor >) -> Option < String > { unimplemented ! () } async fn float_arg_field (& self , float_arg : Option < f64 >) -> Option < String > { unimplemented ! () } async fn id_arg_field (& self , id_arg : Option < ID >) -> Option < String > { unimplemented ! () } async fn string_list_arg_field (& self , string_list_arg : Option < Vec < Option < String > > > ,) -> Option < String > { unimplemented ! () } async fn complex_arg_field (& self , complex_arg : Option < ComplexInput >) -> Option < String > { unimplemented ! () } async fn multiple_reqs (& self , req1 : i32 , req2 : i32) -> Option < String > { unimplemented ! () } async fn multiple_opts (& self , # [graphql (default)] opt1 : i32 , # [graphql (default)] opt2 : i32 ,) -> Option < String > { unimplemented ! () } async fn multiple_opt_and_req (& self , req1 : i32 , req2 : i32 , # [graphql (default)] opt1 : i32 , # [graphql (default)] opt2 : i32 ,) -> Option < String > { unimplemented ! () } }
    };
}

impl_160!()