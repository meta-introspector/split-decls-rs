macro_rules! deps {
    () => {
        Visitor!();
        Field!();
        DepthCalculate!();
        VisitMode!();
        VisitorContext!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl < 'ctx > Visitor < 'ctx > for DepthCalculate < '_ > { fn mode (& self) -> VisitMode { VisitMode :: Inline } fn enter_field (& mut self , _ctx : & mut VisitorContext < 'ctx > , _field : & 'ctx Positioned < Field >) { self . current_depth += 1 ; * self . max_depth = (* self . max_depth) . max (self . current_depth) ; } fn exit_field (& mut self , _ctx : & mut VisitorContext < 'ctx > , _field : & 'ctx Positioned < Field >) { self . current_depth -= 1 ; } }
    };
}

impl_320!();