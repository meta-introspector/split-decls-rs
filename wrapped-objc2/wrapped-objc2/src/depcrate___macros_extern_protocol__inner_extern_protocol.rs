// Generated macro for __inner_extern_protocol (macro)
macro_rules! Depcrate___macros_extern_protocol__inner_extern_protocol {
() => {
// Module: crate::__macros::extern_protocol
// Provides: {"__inner_extern_protocol"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __inner_extern_protocol { (($ protocol : ident) ($ protocol_definition : item) ($ ($ superclasses : tt) *) ($ ($ thread_kind : tt) *) ($ ($ name : tt) *) ($ ($ derives : tt) *) ($ ($ attr_protocol : tt) *) ($ ($ attr_impl : tt) *)) => { $ ($ attr_protocol) * $ protocol_definition $ ($ attr_impl) * unsafe impl < T > $ protocol for $ crate :: runtime :: ProtocolObject < T > where T : ?$ crate :: __macros :: Sized + $ protocol { } $ ($ attr_impl) * unsafe impl $ crate :: ProtocolType for dyn $ protocol { const NAME : &'static $ crate :: __macros :: str = $ crate :: __fallback_if_not_set ! { ($ ($ name) *) ($ crate :: __macros :: stringify ! ($ protocol)) } ; const __INNER : () = () ; } $ ($ attr_impl) * unsafe impl < T > $ crate :: runtime :: ImplementedBy < T > for dyn $ protocol where T : ?$ crate :: __macros :: Sized + $ crate :: Message + $ protocol { const __INNER : () = () ; } $ crate :: __extern_protocol_check_no_super ! ($ ($ superclasses) *) ; $ crate :: __extern_protocol_check_no_thread_kind ! ($ ($ thread_kind) *) ; $ crate :: __extern_protocol_check_no_derives ! ($ ($ derives) *) ; } ; }
};
}
