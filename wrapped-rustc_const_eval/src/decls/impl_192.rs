macro_rules! deps {
    () => {
        ReportErrorExt!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl ReportErrorExt for UnsupportedOpInfo { fn diagnostic_message (& self) -> DiagMessage { use crate :: fluent_generated :: * ; match self { UnsupportedOpInfo :: Unsupported (s) => s . clone () . into () , UnsupportedOpInfo :: ExternTypeField => const_eval_extern_type_field , UnsupportedOpInfo :: UnsizedLocal => const_eval_unsized_local , UnsupportedOpInfo :: ReadPartialPointer (_) => const_eval_partial_pointer_read , UnsupportedOpInfo :: ReadPointerAsInt (_) => const_eval_read_pointer_as_int , UnsupportedOpInfo :: ThreadLocalStatic (_) => const_eval_thread_local_static , UnsupportedOpInfo :: ExternStatic (_) => const_eval_extern_static , } } fn add_args < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { use UnsupportedOpInfo :: * ; use crate :: fluent_generated :: * ; if let ReadPointerAsInt (_) | ReadPartialPointer (_) = self { diag . help (const_eval_ptr_as_bytes_1) ; diag . help (const_eval_ptr_as_bytes_2) ; } match self { UnsizedLocal | UnsupportedOpInfo :: ExternTypeField | Unsupported (_) | ReadPointerAsInt (_) => { } ReadPartialPointer (ptr) => { diag . arg ("ptr" , ptr) ; } ThreadLocalStatic (did) | ExternStatic (did) => rustc_middle :: ty :: tls :: with (| tcx | { diag . arg ("did" , tcx . def_path_str (did)) ; }) , } } }
    };
}

impl_192!();