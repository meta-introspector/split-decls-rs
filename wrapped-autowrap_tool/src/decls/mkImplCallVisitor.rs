macro_rules! deps {
    () => {
        ImplCallVisitor!();
    };
}

macro_rules! mkImplCallVisitor {
    () => {
        deps!();
        macro_rules ! mkImplCallVisitor { (calls : $ init_calls : expr) => { ImplCallVisitor :: new ($ init_calls) } ; }
    };
}

mkImplCallVisitor!()