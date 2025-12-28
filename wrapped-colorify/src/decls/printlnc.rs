macro_rules! printlnc {
    () => {
        # [doc = " `println!` with color."] # [doc = ""] # [doc = " #### Usage"] # [doc = ""] # [doc = " `printlnc!(orange: \"Number of baggies filled while walking dogs: {}\", bag_count);`"] # [doc = ""] # [doc = " See [`colorify!` docs](/colorify/colorify/macro.colorify!.html)"] # [doc = " for a current list of colors."] # [macro_export] macro_rules ! printlnc { ($ c : ident) => (print ! (colorify ! ($ c))) ; ($ c : ident :) => (print ! (colorify ! ($ c))) ; ($ c : ident : $ fmt : expr) => (print ! (concat ! (colorify ! ($ c : $ fmt) , "\n"))) ; ($ c : ident : $ fmt : expr , $ ($ arg : tt) *) => (print ! (concat ! (colorify ! ($ c : $ fmt) , "\n") , $ ($ arg) *)) ; }
    };
}

printlnc!();