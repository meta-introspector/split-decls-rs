// Generated macro for exit_definition (function)
macro_rules! Depcrate_validation_visitorexit_definition {
() => {
// Module: crate::validation::visitor
// Provides: {"exit_definition"}
// Dependencies: {}
fn exit_definition < 'a , S , V > (v : & mut V , ctx : & mut ValidatorContext < 'a , S > , def : & 'a Definition < S >) where S : ScalarValue , V : Visitor < 'a , S > , { match * def { Definition :: Operation (ref op) => v . exit_operation_definition (ctx , op) , Definition :: Fragment (ref f) => v . exit_fragment_definition (ctx , f) , } }
};
}
