macro_rules! deps {
    () => {
        VisitorContext!();
        Field!();
        ServerResult!();
    };
}

macro_rules! ComputeComplexityFn {
    () => {
        deps!();
        type ComputeComplexityFn = fn (& VisitorContext < '_ > , & [Positioned < VariableDefinition >] , & Field , usize ,) -> ServerResult < usize > ;
    };
}

ComputeComplexityFn!();