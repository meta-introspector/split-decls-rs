// Generated macro for system_attribute (function)
macro_rules! Depcrate_systemsystem_attribute {
() => {
// Module: crate::system
// Provides: {"system_attribute"}
// Dependencies: {}
pub fn system_attribute (attr : Option < Ident > , func : ItemFn) -> Result < Item > { let errno = parse_attr (attr) ? ; validate_vis (& func . vis) ? ; let sig = parse_sig (& func . sig) ? ; validate_attrs (& func . attrs) ? ; let func = emit_func (func , & sig , errno) ? ; Ok (Item :: Fn (func)) }
};
}
