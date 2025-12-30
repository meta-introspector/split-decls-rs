// Generated macro for get_factory (function)
macro_rules! Depcrateget_factory {
() => {
// Module: crate
// Provides: {"get_factory"}
// Dependencies: {}
# [test] fn get_factory () { factory :: < Instance , IActivationFactory > () . unwrap () ; let error = factory :: < Instance , IInstance > () . unwrap_err () ; assert_eq ! (error . code () , E_NOINTERFACE) ; let error = factory :: < Missing , IActivationFactory > () . unwrap_err () ; assert_eq ! (error . code () , REGDB_E_CLASSNOTREG) ; }
};
}
