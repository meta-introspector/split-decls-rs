// Generated macro for impl_81 (impl)
macro_rules! Depcrate_curveimpl_81 {
() => {
// Module: crate::curve
// Provides: {"impl_81"}
// Dependencies: {}
impl Script for Properties { fn script (& self) -> String { let mut script = if let Some (axes) = self . axes { format ! ("axes {} " , axes . display ()) } else { String :: new () } ; script . push_str (& format ! ("with {} " , self . style . display ())) ; script . push_str (& format ! ("lt {} " , self . line_type . display ())) ; if let Some (lw) = self . linewidth { script . push_str (& format ! ("lw {} " , lw)) } if let Some (color) = self . color { script . push_str (& format ! ("lc rgb '{}' " , color . display ())) } if let Some (pt) = self . point_type { script . push_str (& format ! ("pt {} " , pt . display ())) } if let Some (ps) = self . point_size { script . push_str (& format ! ("ps {} " , ps)) } if let Some (ref label) = self . label { script . push_str ("title '") ; script . push_str (label) ; script . push ('\'') } else { script . push_str ("notitle") } script } }
};
}
