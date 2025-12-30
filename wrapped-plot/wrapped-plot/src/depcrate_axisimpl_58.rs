// Generated macro for impl_58 (impl)
macro_rules! Depcrate_axisimpl_58 {
() => {
// Module: crate::axis
// Provides: {"impl_58"}
// Dependencies: {}
impl Script for (Axis , & Properties) { fn script (& self) -> String { let & (axis , properties) = self ; let axis_ = axis . display () ; let mut script = if properties . hidden { return format ! ("unset {}tics\n" , axis_) ; } else { format ! ("set {}tics nomirror " , axis_) } ; if let Some (ref tics) = properties . tics { script . push_str (& format ! ("({})" , tics)) } script . push ('\n') ; if let Some (ref label) = properties . label { script . push_str (& format ! ("set {}label '{}'\n" , axis_ , label)) } if let Some ((low , high)) = properties . range { script . push_str (& format ! ("set {}range [{}:{}]\n" , axis_ , low , high)) } if properties . logarithmic { script . push_str (& format ! ("set logscale {}\n" , axis_)) ; } for (grid , properties) in properties . grids . iter () { script . push_str (& (axis , grid , properties) . script ()) ; } script } }
};
}
