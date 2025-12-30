// Generated macro for enter_definition (function)
macro_rules! Depcrate_validation_visitorenter_definition {
() => {
// Module: crate::validation::visitor
// Provides: {"enter_definition"}
// Dependencies: {}
fn enter_definition < 'a , S , V > (v : & mut V , ctx : & mut ValidatorContext < 'a , S > , def : & 'a Definition < S >) where S : ScalarValue , V : Visitor < 'a , S > , { match * def { Definition :: Operation (ref op) => v . enter_operation_definition (ctx , op) , Definition :: Fragment (ref f) => v . enter_fragment_definition (ctx , f) , } }
};
}
