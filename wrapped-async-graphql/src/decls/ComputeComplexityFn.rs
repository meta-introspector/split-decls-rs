macro_rules! deps {
    () => {
        Field!();
        ServerResult!();
        VisitorContext!();
    };
}

macro_rules! ComputeComplexityFn {
    () => {
        deps!();
        type ComputeComplexityFn = fn (& VisitorContext < '_ > , & [Positioned < VariableDefinition >] , & Field , usize ,) -> ServerResult < usize > ;
    };
}

ComputeComplexityFn!()