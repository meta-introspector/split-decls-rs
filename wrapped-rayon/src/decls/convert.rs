macro_rules! convert {
    () => {
        macro_rules ! convert { ($ iter : ident . $ method : ident ($ ($ arg : expr) ,*)) => { if let Some ((start , end)) = $ iter . bounds () { if let Some (end) = end . checked_add (1) { (start .. end) . into_par_iter () .$ method ($ ($ arg) ,*) } else { (start .. end) . into_par_iter () . chain (once (end)) .$ method ($ ($ arg) ,*) } } else { empty ::< Self > () .$ method ($ ($ arg) ,*) } } ; }
    };
}

convert!();