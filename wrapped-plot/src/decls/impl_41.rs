macro_rules! deps {
    () => {
        Properties!();
        Script!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl Script for Properties { fn script (& self) -> String { let mut script = String :: from ("with candlesticks ") ; script . push_str (& format ! ("lt {} " , self . line_type . display ())) ; if let Some (lw) = self . linewidth { script . push_str (& format ! ("lw {} " , lw)) } if let Some (color) = self . color { script . push_str (& format ! ("lc rgb '{}' " , color . display ())) ; } if let Some (ref label) = self . label { script . push_str ("title '") ; script . push_str (label) ; script . push ('\'') } else { script . push_str ("notitle") } script } }
    };
}

impl_41!()