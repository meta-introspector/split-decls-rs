macro_rules! deps {
    () => {
        Script!();
        Properties!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl Script for Properties { fn script (& self) -> String { let mut script = format ! ("with {} " , self . style . display ()) ; script . push_str (& format ! ("lt {} " , self . line_type . display ())) ; if let Some (lw) = self . linewidth { script . push_str (& format ! ("lw {} " , lw)) } if let Some (color) = self . color { script . push_str (& format ! ("lc rgb '{}' " , color . display ())) } if let Some (pt) = self . point_type { script . push_str (& format ! ("pt {} " , pt . display ())) } if let Some (ps) = self . point_size { script . push_str (& format ! ("ps {} " , ps)) } if let Some (ref label) = self . label { script . push_str ("title '") ; script . push_str (label) ; script . push ('\'') } else { script . push_str ("notitle") } script } }
    };
}

impl_67!();