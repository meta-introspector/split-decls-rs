// Generated macro for tuple_trait_impl (macro)
macro_rules! Depcrate_sequencetuple_trait_impl {
() => {
// Module: crate::sequence
// Provides: {"tuple_trait_impl"}
// Dependencies: {}
macro_rules ! tuple_trait_impl (($ ($ name : ident $ ty : ident) ,+) => (# [allow (deprecated)] impl < Input : Clone , $ ($ ty) ,+ , Error : ParseError < Input >, $ ($ name : Parser < Input , Output = $ ty , Error = Error >) ,+ > Tuple < Input , ($ ($ ty) ,+) , Error > for ($ ($ name) ,+) { fn parse_tuple (& mut self , input : Input) -> IResult < Input , ($ ($ ty) ,+) , Error > { tuple_trait_inner ! (0 , self , input , () , $ ($ name) +) } }) ;) ;
};
}
