macro_rules! printc {
    () => {
        # [doc = " `print!` with all the glorious colors of the the ANSI rainbow."] # [doc = ""] # [doc = " Watch out for the leprechaun at the end of that rainbow. Seriously."] # [doc = ""] # [doc = " #### Usage"] # [doc = ""] # [doc = " `printc!(yellow: \"Number of banana peels on head: {}\", hat_height);`"] # [doc = ""] # [doc = " See [`colorify!` docs](/colorify/colorify/macro.colorify!.html)"] # [doc = " for a current list of colors."] # [macro_export] macro_rules ! printc { ($ c : ident) => (print ! (colorify ! ($ c))) ; ($ c : ident :) => (print ! (colorify ! ($ c))) ; ($ c : ident : $ fmt : expr) => (print ! (colorify ! ($ c : $ fmt))) ; ($ c : ident : $ fmt : expr , $ ($ arg : tt) *) => (print ! (colorify ! ($ c : $ fmt) , $ ($ arg) *)) ; }
    };
}

printc!()