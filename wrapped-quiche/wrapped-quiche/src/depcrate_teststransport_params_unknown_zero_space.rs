// Generated macro for transport_params_unknown_zero_space (function)
macro_rules! Depcrate_teststransport_params_unknown_zero_space {
() => {
// Module: crate::tests
// Provides: {"transport_params_unknown_zero_space"}
// Dependencies: {}
# [test] fn transport_params_unknown_zero_space () { let mut unknown_params : UnknownTransportParameters = UnknownTransportParameters { capacity : 0 , parameters : vec ! [] , } ; let massive_unknown_param = UnknownTransportParameter :: < & [u8] > { id : 5 , value : & [0xau8 ; 280] , } ; assert ! (unknown_params . push (massive_unknown_param) . is_err ()) ; assert ! (unknown_params . capacity == 0) ; assert ! (unknown_params . parameters . is_empty ()) ; }
};
}
