// Generated macro for impl_164 (impl)
macro_rules! Depcrate___macros_define_class_checksimpl_164 {
() => {
// Module: crate::__macros::define_class::checks
// Provides: {"impl_164"}
// Dependencies: {}
impl < T : DefinedClass > ClassBuilderHelper < T > { # [inline] pub fn add_protocol_methods < P > (& mut self) -> ClassProtocolMethodsBuilder < '_ , T > where P : ? Sized + ProtocolType , { let protocol = P :: protocol () ; if let Some (protocol) = protocol { self . builder . add_protocol (protocol) ; } # [cfg (debug_assertions)] { ClassProtocolMethodsBuilder { builder : self , protocol , required_instance_methods : protocol . map (| p | p . method_descriptions (true)) . unwrap_or_default () , optional_instance_methods : protocol . map (| p | p . method_descriptions (false)) . unwrap_or_default () , registered_instance_methods : HashSet :: new () , required_class_methods : protocol . map (| p | p . class_method_descriptions (true)) . unwrap_or_default () , optional_class_methods : protocol . map (| p | p . class_method_descriptions (false)) . unwrap_or_default () , registered_class_methods : HashSet :: new () , } } # [cfg (not (debug_assertions))] { ClassProtocolMethodsBuilder { builder : self } } } # [inline] pub unsafe fn add_method < F > (& mut self , sel : Sel , func : F) where F : MethodImplementation < Callee = T > , { unsafe { self . builder . add_method (sel , func) } } # [inline] pub unsafe fn add_class_method < F > (& mut self , sel : Sel , func : F) where F : MethodImplementation < Callee = AnyClass > , { unsafe { self . builder . add_class_method (sel , func) } } }
};
}
