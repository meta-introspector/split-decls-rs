macro_rules! ComplexityCalculate {
    () => {
        pub struct ComplexityCalculate < 'ctx , 'a > { pub complexity : & 'a mut usize , pub complexity_stack : Vec < usize > , pub variable_definition : Option < & 'ctx [Positioned < VariableDefinition >] > , }
    };
}

ComplexityCalculate!();