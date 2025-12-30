// Generated macro for walk_param_bound (function)
macro_rules! Depcrate_intravisitwalk_param_bound {
() => {
// Module: crate::intravisit
// Provides: {"walk_param_bound"}
// Dependencies: {}
pub fn walk_param_bound < 'v , V : Visitor < 'v > > (visitor : & mut V , bound : & 'v GenericBound < 'v > ,) -> V :: Result { match * bound { GenericBound :: Trait (ref typ) => visitor . visit_poly_trait_ref (typ) , GenericBound :: Outlives (ref lifetime) => visitor . visit_lifetime (lifetime) , GenericBound :: Use (args , _) => { walk_list ! (visitor , visit_precise_capturing_arg , args) ; V :: Result :: output () } } }
};
}
