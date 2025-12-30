// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
# [rocket :: async_trait] impl < 'f , S > FromForm < 'f > for GraphQLRequest < S > where S : ScalarValue + Send , { type Context = GraphQLContext < 'f , S > ; fn init (opts : Options) -> Self :: Context { GraphQLContext { opts , query : None , operation_name : None , variables : None , errors : Errors :: new () , } } fn push_value (ctx : & mut Self :: Context , field : ValueField < 'f >) { match field . name . key () . map (| key | key . as_str ()) { Some ("query") => ctx . query (field . value . into ()) , Some ("operation_name" | "operationName") => ctx . operation_name (field . value . into ()) , Some ("variables") => ctx . variables (field . value . into ()) , Some (key) => { if ctx . opts . strict { let error = Error :: from (ErrorKind :: Unknown) . with_name (key) ; ctx . errors . push (error) } } None => { if ctx . opts . strict { let error = Error :: from (ErrorKind :: Unexpected) ; ctx . errors . push (error) } } } } async fn push_data (ctx : & mut Self :: Context , field : DataField < 'f , '_ >) { if ctx . opts . strict { let error = Error :: from (ErrorKind :: Unexpected) . with_name (field . name) ; ctx . errors . push (error) } } fn finalize (mut ctx : Self :: Context) -> rocket :: form :: Result < 'f , Self > { if ctx . query . is_none () { let error = Error :: from (ErrorKind :: Missing) . with_name ("query") ; ctx . errors . push (error) } match ctx . errors . is_empty () { true => Ok (GraphQLRequest (GraphQLBatchRequest :: Single (http :: GraphQLRequest :: new (ctx . query . unwrap () , ctx . operation_name , ctx . variables) ,))) , false => Err (ctx . errors) , } } }
};
}
