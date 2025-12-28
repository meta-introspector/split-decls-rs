macro_rules! deps {
    () => {
        DefineOpaqueTypes!();
        LatticeOp!();
        LatticeOpKind!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < 'infcx , 'tcx > LatticeOp < 'infcx , 'tcx > { fn relate_bound (& mut self , v : Ty < 'tcx > , a : Ty < 'tcx > , b : Ty < 'tcx >) -> RelateResult < 'tcx , () > { let at = self . infcx . at (& self . trace . cause , self . param_env) ; match self . kind { LatticeOpKind :: Glb => { self . obligations . extend (at . sub (DefineOpaqueTypes :: Yes , v , a) ? . into_obligations ()) ; self . obligations . extend (at . sub (DefineOpaqueTypes :: Yes , v , b) ? . into_obligations ()) ; } LatticeOpKind :: Lub => { self . obligations . extend (at . sub (DefineOpaqueTypes :: Yes , a , v) ? . into_obligations ()) ; self . obligations . extend (at . sub (DefineOpaqueTypes :: Yes , b , v) ? . into_obligations ()) ; } } Ok (()) } }
    };
}

impl_168!();