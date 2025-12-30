// Generated macro for visit (function)
macro_rules! Depcrate_validation_visitorvisit {
() => {
// Module: crate::validation::visitor
// Provides: {"visit"}
// Dependencies: {}
# [doc (hidden)] pub fn visit < 'a , A , B , S > (v : & mut MultiVisitorCons < A , B > , ctx : & mut ValidatorContext < 'a , S > , d : & 'a Document < S > ,) where S : ScalarValue , MultiVisitorCons < A , B > : Visitor < 'a , S > , { v . enter_document (ctx , d) ; visit_definitions (v , ctx , d) ; v . exit_document (ctx , d) ; }
};
}
