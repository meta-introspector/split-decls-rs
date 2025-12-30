// Generated macro for impl_54 (impl)
macro_rules! Depcrate_lifetimeimpl_54 {
() => {
// Module: crate::lifetime
// Provides: {"impl_54"}
// Dependencies: {}
impl VisitMut for CollectLifetimes { fn visit_receiver_mut (& mut self , arg : & mut Receiver) { if let Some ((reference , lifetime)) = & mut arg . reference { self . visit_opt_lifetime (reference , lifetime) ; } else { visit_mut :: visit_type_mut (self , & mut arg . ty) ; } } fn visit_type_reference_mut (& mut self , ty : & mut TypeReference) { self . visit_opt_lifetime (& ty . and_token , & mut ty . lifetime) ; visit_mut :: visit_type_reference_mut (self , ty) ; } fn visit_generic_argument_mut (& mut self , gen : & mut GenericArgument) { if let GenericArgument :: Lifetime (lifetime) = gen { self . visit_lifetime (lifetime) ; } visit_mut :: visit_generic_argument_mut (self , gen) ; } }
};
}
