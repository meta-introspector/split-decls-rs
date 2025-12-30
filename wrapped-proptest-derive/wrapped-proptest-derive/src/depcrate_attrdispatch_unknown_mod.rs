// Generated macro for dispatch_unknown_mod (function)
macro_rules! Depcrate_attrdispatch_unknown_mod {
() => {
// Module: crate::attr
// Provides: {"dispatch_unknown_mod"}
// Dependencies: {}
fn dispatch_unknown_mod (ctx : Ctx , name : & str) { match name { "no_bounds" => error :: did_you_mean (ctx , name , "no_bound") , "weights" | "weighted" => error :: did_you_mean (ctx , name , "weight") , "strat" | "strategies" => error :: did_you_mean (ctx , name , "strategy") , "values" | "valued" | "fix" | "fixed" => { error :: did_you_mean (ctx , name , "value") } "regexes" | "regexp" | "re" => error :: did_you_mean (ctx , name , "regex") , "param" | "parameters" => error :: did_you_mean (ctx , name , "params") , "no_param" | "no_parameters" => { error :: did_you_mean (ctx , name , "no_params") } name => error :: unkown_modifier (ctx , name) , } }
};
}
