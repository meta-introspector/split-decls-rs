macro_rules! deps {
    () => {
        ArrayVec!();
        EvaluationStorage!();
        EvaluationResult!();
        Result!();
        Expression!();
        EvaluationState!();
        Value!();
        Reader!();
        Encoding!();
        StoreOnHeap!();
    };
}

macro_rules! Evaluation {
    () => {
        deps!();
        # [doc = " A DWARF expression evaluator."] # [doc = ""] # [doc = " # Usage"] # [doc = " A DWARF expression may require additional data to produce a final result,"] # [doc = " such as the value of a register or a memory location.  Once initial setup"] # [doc = " is complete (i.e. `set_initial_value()`, `set_object_address()`) the"] # [doc = " consumer calls the `evaluate()` method.  That returns an `EvaluationResult`,"] # [doc = " which is either `EvaluationResult::Complete` or a value indicating what"] # [doc = " data is needed to resume the `Evaluation`.  The consumer is responsible for"] # [doc = " producing that data and resuming the computation with the correct method,"] # [doc = " as documented for `EvaluationResult`.  Only once an `EvaluationResult::Complete`"] # [doc = " is returned can the consumer call `result()`."] # [doc = ""] # [doc = " This design allows the consumer of `Evaluation` to decide how and when to"] # [doc = " produce the required data and resume the computation.  The `Evaluation` can"] # [doc = " be driven synchronously (as shown below) or by some asynchronous mechanism"] # [doc = " such as futures."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```rust,no_run"] # [doc = " use gimli::{Evaluation, EvaluationResult, Expression};"] # [doc = " # let bytecode = gimli::EndianSlice::new(&[], gimli::LittleEndian);"] # [doc = " # let encoding = unimplemented!();"] # [doc = " # let get_register_value = |_, _| gimli::Value::Generic(42);"] # [doc = " # let get_frame_base = || 0xdeadbeef;"] # [doc = ""] # [doc = " let mut eval = Evaluation::new(bytecode, encoding);"] # [doc = " let mut result = eval.evaluate().unwrap();"] # [doc = " while result != EvaluationResult::Complete {"] # [doc = "   match result {"] # [doc = "     EvaluationResult::RequiresRegister { register, base_type } => {"] # [doc = "       let value = get_register_value(register, base_type);"] # [doc = "       result = eval.resume_with_register(value).unwrap();"] # [doc = "     },"] # [doc = "     EvaluationResult::RequiresFrameBase => {"] # [doc = "       let frame_base = get_frame_base();"] # [doc = "       result = eval.resume_with_frame_base(frame_base).unwrap();"] # [doc = "     },"] # [doc = "     _ => unimplemented!(),"] # [doc = "   };"] # [doc = " }"] # [doc = ""] # [doc = " let result = eval.result();"] # [doc = " println!(\"{:?}\", result);"] # [doc = " ```"] # [derive (Debug)] pub struct Evaluation < R : Reader , S : EvaluationStorage < R > = StoreOnHeap > { bytecode : R , encoding : Encoding , object_address : Option < u64 > , max_iterations : Option < u32 > , iteration : u32 , state : EvaluationState < R > , addr_mask : u64 , stack : ArrayVec < S :: Stack > , pc : R , expression_stack : ArrayVec < S :: ExpressionStack > , value_result : Option < Value > , result : ArrayVec < S :: Result > , }
    };
}

Evaluation!()