macro_rules! deps {
    () => {
        Script!();
        Position!();
        Properties!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl Script for Properties { fn script (& self) -> String { let mut script = if self . hidden { return String :: from ("set key off\n") ; } else { String :: from ("set key on ") } ; match self . position { None => { } Some (Position :: Inside (v , h)) => { script . push_str (& format ! ("inside {} {} " , v . display () , h . display ())) } Some (Position :: Outside (v , h)) => { script . push_str (& format ! ("outside {} {} " , v . display () , h . display ())) } } if let Some (stacked) = self . stacked { script . push_str (stacked . display ()) ; script . push (' ') ; } if let Some (justification) = self . justification { script . push_str (justification . display ()) ; script . push (' ') ; } if let Some (order) = self . order { script . push_str (order . display ()) ; script . push (' ') ; } if let Some (ref title) = self . title { script . push_str (& format ! ("title '{}' " , title)) } if self . boxed { script . push_str ("box ") } script . push ('\n') ; script } }
    };
}

impl_98!();