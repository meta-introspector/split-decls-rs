// Generated macro for Generator (struct)
macro_rules! Depcrate_abiGenerator {
() => {
// Module: crate::abi
// Provides: {"Generator"}
// Dependencies: {}
struct Generator < 'a , B : Bindgen > { bindgen : & 'a mut B , resolve : & 'a Resolve , operands : Vec < B :: Operand > , results : Vec < B :: Operand > , stack : Vec < B :: Operand > , return_pointer : Option < B :: Operand > , realloc : Option < Realloc > , }
};
}
