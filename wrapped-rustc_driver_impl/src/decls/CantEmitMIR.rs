macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! CantEmitMIR {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (driver_impl_cant_emit_mir)] pub struct CantEmitMIR { pub error : std :: io :: Error , }
    };
}

CantEmitMIR!()