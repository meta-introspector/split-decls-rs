// Generated macro for visit_definitions (function)
macro_rules! Depcrate_validation_visitorvisit_definitions {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_definitions"}
// Dependencies: {}
fn visit_definitions < 'a , S , V > (v : & mut V , ctx : & mut ValidatorContext < 'a , S > , d : & 'a [Definition < S >]) where S : ScalarValue , V : Visitor < 'a , S > , { for def in d { let def_type = match def { Definition :: Fragment (Spanning { item : Fragment { type_condition : Spanning { item : name , .. } , .. } , .. }) => Some (BorrowedType :: non_null (name)) , Definition :: Operation (Spanning { item : Operation { operation_type : OperationType :: Query , .. } , .. }) => Some (BorrowedType :: non_null (ctx . schema . concrete_query_type () . name () . unwrap () ,)) , Definition :: Operation (Spanning { item : Operation { operation_type : OperationType :: Mutation , .. } , .. }) => ctx . schema . concrete_mutation_type () . map (| t | BorrowedType :: non_null (t . name () . unwrap ())) , Definition :: Operation (Spanning { item : Operation { operation_type : OperationType :: Subscription , .. } , .. }) => ctx . schema . concrete_subscription_type () . map (| t | BorrowedType :: non_null (t . name () . unwrap ())) , } ; ctx . with_pushed_type (def_type , | ctx | { enter_definition (v , ctx , def) ; visit_definition (v , ctx , def) ; exit_definition (v , ctx , def) ; }) ; } }
};
}
