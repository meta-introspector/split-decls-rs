// Generated macro for transport_params_unknown_is_reserved (function)
macro_rules! Depcrate_teststransport_params_unknown_is_reserved {
() => {
// Module: crate::tests
// Provides: {"transport_params_unknown_is_reserved"}
// Dependencies: {}
# [test] fn transport_params_unknown_is_reserved () { let reserved_unknown_param = UnknownTransportParameter :: < & [u8] > { id : 31 * 17 + 27 , value : & [0xau8 ; 280] , } ; let not_reserved_unknown_param = UnknownTransportParameter :: < & [u8] > { id : 32 * 17 + 27 , value : & [0xau8 ; 280] , } ; assert ! (reserved_unknown_param . is_reserved ()) ; assert ! (! not_reserved_unknown_param . is_reserved ()) ; }
};
}
