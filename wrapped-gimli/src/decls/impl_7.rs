macro_rules! deps {
    () => {
        LineEncoding!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Default for LineEncoding { fn default () -> Self { LineEncoding { minimum_instruction_length : 1 , maximum_operations_per_instruction : 1 , default_is_stmt : true , line_base : - 5 , line_range : 14 , } } }
    };
}

impl_7!()