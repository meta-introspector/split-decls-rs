macro_rules! deps {
    () => {
        Script!();
        Properties!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl Script for Properties { fn script (& self) -> String { let mut script = if let Some (axes) = self . axes { format ! ("axes {} " , axes . display ()) } else { String :: new () } ; script . push_str ("with filledcurves ") ; script . push_str ("fillstyle ") ; if let Some (opacity) = self . opacity { script . push_str (& format ! ("solid {} " , opacity)) } script . push_str ("noborder ") ; if let Some (color) = self . color { script . push_str (& format ! ("lc rgb '{}' " , color . display ())) ; } if let Some (ref label) = self . label { script . push_str ("title '") ; script . push_str (label) ; script . push ('\'') } else { script . push_str ("notitle") } script } }
    };
}

impl_82!();