// Generated macro for permutation_trait_impl (macro)
macro_rules! Depcrate_branchpermutation_trait_impl {
() => {
// Module: crate::branch
// Provides: {"permutation_trait_impl"}
// Dependencies: {}
macro_rules ! permutation_trait_impl (($ ($ name : ident $ ty : ident $ item : ident) ,+) => (impl < Input , Error , $ ($ ty) ,+ , $ ($ name) ,+ > Parser < Input > for Permutation < ($ ($ name) ,+) , Error > where Input : Clone , Error : ParseError < Input >, $ ($ name : Parser < Input , Output = $ ty , Error = Error >) ,+ { type Output = ($ ($ ty) ,+) ; type Error = Error ; # [inline (always)] fn process < OM : crate :: OutputMode > (& mut self , mut input : Input ,) -> crate :: PResult < OM , Input , Self :: Output , Self :: Error > { let mut res = OM :: Output :: bind (|| ($ (Option ::<$ ty >:: None) ,+)) ; $ (let mut $ item = false ;) + loop { let mut err : Option << OM :: Error as Mode >:: Output < Error >> = None ; permutation_trait_inner ! (0 , self , input , res , err , $ ($ item) +) ; if let Some (err) = err { return Err (Err :: Error (OM :: Error :: map (err , | err | Error :: append (input , ErrorKind :: Permutation , err)))) ; } return Ok ((input , OM :: Output :: map (res , | res | { match res { ($ (Some ($ item)) ,+) => ($ ($ item) ,+) , _ => unreachable ! () , } }))) } } }) ;) ;
};
}
