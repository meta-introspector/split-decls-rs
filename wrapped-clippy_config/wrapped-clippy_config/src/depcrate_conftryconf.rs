// Generated macro for TryConf (struct)
macro_rules! Depcrate_confTryConf {
() => {
// Module: crate::conf
// Provides: {"TryConf"}
// Dependencies: {}
# [doc = " Conf with parse errors"] # [derive (Default)] struct TryConf { conf : Conf , value_spans : HashMap < String , Range < usize > > , errors : Vec < ConfError > , warnings : Vec < ConfError > , }
};
}
