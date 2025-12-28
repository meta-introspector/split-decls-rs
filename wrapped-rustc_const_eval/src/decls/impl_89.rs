macro_rules! deps {
    () => {
        ConstEvalErrKind!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        # [doc = " The errors become [`InterpErrorKind::MachineStop`] when being raised."] impl < 'tcx > Into < InterpErrorInfo < 'tcx > > for ConstEvalErrKind { fn into (self) -> InterpErrorInfo < 'tcx > { err_machine_stop ! (self) . into () } }
    };
}

impl_89!();