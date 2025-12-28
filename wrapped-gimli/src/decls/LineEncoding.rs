macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! LineEncoding {
    () => {
        deps!();
        # [doc = " Encoding parameters for a line number program."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct LineEncoding { # [doc = " The size in bytes of the smallest target machine instruction."] pub minimum_instruction_length : u8 , # [doc = " The maximum number of individual operations that may be encoded in an"] # [doc = " instruction."] pub maximum_operations_per_instruction : u8 , # [doc = " The initial value of the `is_stmt` register."] pub default_is_stmt : bool , # [doc = " The minimum value which a special opcode can add to the line register."] pub line_base : i8 , # [doc = " The range of values which a special opcode can add to the line register."] pub line_range : u8 , }
    };
}

LineEncoding!()