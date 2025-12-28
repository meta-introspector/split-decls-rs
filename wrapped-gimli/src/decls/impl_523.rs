macro_rules! deps {
    () => {
        Evaluation!();
        Encoding!();
        Piece!();
        Reader!();
        EvaluationState!();
    };
}

macro_rules! impl_523 {
    () => {
        deps!();
        # [cfg (feature = "read")] impl < R : Reader > Evaluation < R > { # [doc = " Create a new DWARF expression evaluator."] # [doc = ""] # [doc = " The new evaluator is created without an initial value, without"] # [doc = " an object address, and without a maximum number of iterations."] pub fn new (bytecode : R , encoding : Encoding) -> Self { Self :: new_in (bytecode , encoding) } # [doc = " Get the result of this `Evaluation`."] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if this `Evaluation` has not been driven to completion."] pub fn result (self) -> Vec < Piece < R > > { match self . state { EvaluationState :: Complete => self . result . into_vec () , _ => { panic ! ("Called `Evaluation::result` on an `Evaluation` that has not been completed") } } } }
    };
}

impl_523!();