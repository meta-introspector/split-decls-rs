mkuse!{use cranelift_codegen :: ir :: condcodes :: IntCC ;}
mkuse!{use cranelift_codegen :: ir :: { InstructionData , Opcode , Value , ValueDef } ;}
mkuse!{use cranelift_frontend :: FunctionBuilder ;}

macro_rules! maybe_unwrap_bool_not_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_unwrap_bool_not in module {}", module_path!());
    };
}

mkfn!{
    maybe_unwrap_bool_not_introspect!();
    # [doc = " If the given value was produced by the lowering of `Rvalue::Not` return the input and true,"] # [doc = " otherwise return the given value and false."] pub (crate) fn maybe_unwrap_bool_not (bcx : & mut FunctionBuilder < '_ > , arg : Value) -> (Value , bool) { if let ValueDef :: Result (arg_inst , 0) = bcx . func . dfg . value_def (arg) { match bcx . func . dfg . insts [arg_inst] { InstructionData :: IntCompareImm { opcode : Opcode :: IcmpImm , cond : IntCC :: Equal , arg , imm , } if imm . bits () == 0 => (arg , true) , _ => (arg , false) , } } else { (arg , false) } }
}

macro_rules! maybe_known_branch_taken_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_known_branch_taken in module {}", module_path!());
    };
}

mkfn!{
    maybe_known_branch_taken_introspect!();
    # [doc = " Returns whether the branch is statically known to be taken or `None` if it isn't statically known."] pub (crate) fn maybe_known_branch_taken (bcx : & FunctionBuilder < '_ > , arg : Value , test_zero : bool ,) -> Option < bool > { let arg_inst = if let ValueDef :: Result (arg_inst , 0) = bcx . func . dfg . value_def (arg) { arg_inst } else { return None ; } ; match bcx . func . dfg . insts [arg_inst] { InstructionData :: UnaryImm { opcode : Opcode :: Iconst , imm } => { if test_zero { Some (imm . bits () == 0) } else { Some (imm . bits () != 0) } } _ => None , } }
}