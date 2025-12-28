macro_rules! deps {
    () => {
        ComplexityCalculate!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < 'a > ComplexityCalculate < '_ , 'a > { pub fn new (complexity : & 'a mut usize) -> Self { Self { complexity , complexity_stack : Default :: default () , variable_definition : None , } } }
    };
}

impl_314!();