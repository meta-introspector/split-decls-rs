// Generated macro for LivenessContext (struct)
macro_rules! Depcrate_outputLivenessContext {
() => {
// Module: crate::output
// Provides: {"LivenessContext"}
// Dependencies: {}
# [doc = " Subset of `AllFacts` dedicated to liveness"] struct LivenessContext < T : FactTypes > { var_used_at : Vec < (T :: Variable , T :: Point) > , var_defined_at : Vec < (T :: Variable , T :: Point) > , var_dropped_at : Vec < (T :: Variable , T :: Point) > , use_of_var_derefs_origin : Vec < (T :: Variable , T :: Origin) > , drop_of_var_derefs_origin : Vec < (T :: Variable , T :: Origin) > , }
};
}
