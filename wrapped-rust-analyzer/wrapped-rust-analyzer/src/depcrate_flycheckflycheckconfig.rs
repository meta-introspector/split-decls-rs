// Generated macro for FlycheckConfig (enum)
macro_rules! Depcrate_flycheckFlycheckConfig {
() => {
// Module: crate::flycheck
// Provides: {"FlycheckConfig"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq)] pub (crate) enum FlycheckConfig { CargoCommand { command : String , options : CargoOptions , ansi_color_output : bool , } , CustomCommand { command : String , args : Vec < String > , extra_env : FxHashMap < String , Option < String > > , invocation_strategy : InvocationStrategy , } , }
};
}
