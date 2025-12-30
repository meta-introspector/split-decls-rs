// Generated macro for impl_34 (impl)
macro_rules! Depcrate_confimpl_34 {
() => {
// Module: crate::conf
// Provides: {"impl_34"}
// Dependencies: {}
impl TryConf { fn from_toml_error (file : & SourceFile , error : & toml :: de :: Error) -> Self { Self { conf : Conf :: default () , value_spans : HashMap :: default () , errors : vec ! [ConfError :: from_toml (file , error)] , warnings : vec ! [] , } } }
};
}
