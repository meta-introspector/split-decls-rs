macro_rules! deps {
    () => {
        Diagnostic!();
        ParseTargetMachineConfig!();
        LlvmError!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        impl < G : EmissionGuarantee > Diagnostic < '_ , G > for ParseTargetMachineConfig < '_ > { fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { let diag : Diag < '_ , G > = self . 0 . into_diag (dcx , level) ; let (message , _) = diag . messages . first () . expect ("`LlvmError` with no message") ; let message = dcx . eagerly_translate_to_string (message . clone () , diag . args . iter ()) ; Diag :: new (dcx , level , fluent :: codegen_llvm_parse_target_machine_config) . with_arg ("error" , message) } }
    };
}

impl_377!()