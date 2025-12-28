macro_rules! deps {
    () => {
        ReportErrorExt!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < 'tcx > ReportErrorExt for InterpErrorKind < 'tcx > { fn diagnostic_message (& self) -> DiagMessage { match self { InterpErrorKind :: UndefinedBehavior (ub) => ub . diagnostic_message () , InterpErrorKind :: Unsupported (e) => e . diagnostic_message () , InterpErrorKind :: InvalidProgram (e) => e . diagnostic_message () , InterpErrorKind :: ResourceExhaustion (e) => e . diagnostic_message () , InterpErrorKind :: MachineStop (e) => e . diagnostic_message () , } } fn add_args < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { match self { InterpErrorKind :: UndefinedBehavior (ub) => ub . add_args (diag) , InterpErrorKind :: Unsupported (e) => e . add_args (diag) , InterpErrorKind :: InvalidProgram (e) => e . add_args (diag) , InterpErrorKind :: ResourceExhaustion (e) => e . add_args (diag) , InterpErrorKind :: MachineStop (e) => e . add_args (& mut | name , value | { diag . arg (name , value) ; }) , } } }
    };
}

impl_193!()