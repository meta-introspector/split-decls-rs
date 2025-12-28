macro_rules! deps {
    () => {
        ReportErrorExt!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < 'tcx > ReportErrorExt for InvalidProgramInfo < 'tcx > { fn diagnostic_message (& self) -> DiagMessage { use crate :: fluent_generated :: * ; match self { InvalidProgramInfo :: TooGeneric => const_eval_too_generic , InvalidProgramInfo :: AlreadyReported (_) => const_eval_already_reported , InvalidProgramInfo :: Layout (e) => e . diagnostic_message () , } } fn add_args < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { match self { InvalidProgramInfo :: TooGeneric | InvalidProgramInfo :: AlreadyReported (_) => { } InvalidProgramInfo :: Layout (e) => { let dummy_level = Level :: Bug ; let dummy_diag : Diag < '_ , () > = e . into_diagnostic () . into_diag (diag . dcx , dummy_level) ; for (name , val) in dummy_diag . args . iter () { diag . arg (name . clone () , val . clone ()) ; } dummy_diag . cancel () ; } } } }
    };
}

impl_194!()