// Generated macro for collect_template_args (function)
macro_rules! Depcratecollect_template_args {
() => {
// Module: crate
// Provides: {"collect_template_args"}
// Dependencies: {}
fn collect_template_args (template : & ItemFn) -> HashMap < & Ident , & PatType > { template . sig . inputs . iter () . filter_map (| arg | match arg { syn :: FnArg :: Typed (a) => Some (a) , _ => None , }) . filter_map (| arg | match * arg . pat { syn :: Pat :: Ident (ref id) => Some ((& id . ident , arg)) , _ => None , }) . collect () }
};
}
