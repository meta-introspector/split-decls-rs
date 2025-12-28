macro_rules! deps {
    () => {
        State!();
        TransferFunction!();
        ConstCx!();
        FlowSensitiveAnalysis!();
        Qualif!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < 'mir , 'tcx , Q > FlowSensitiveAnalysis < 'mir , 'tcx , Q > where Q : Qualif , { pub (super) fn new (_ : Q , ccx : & 'mir ConstCx < 'mir , 'tcx >) -> Self { FlowSensitiveAnalysis { ccx , _qualif : PhantomData } } fn transfer_function (& self , state : & 'mir mut State) -> TransferFunction < 'mir , 'tcx , Q > { TransferFunction :: < Q > :: new (self . ccx , state) } }
    };
}

impl_69!();