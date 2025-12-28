macro_rules! deps {
    () => {
        FileCsvReport!();
    };
}

macro_rules! reports_impl {
    () => {
        deps!();
        macro_rules ! reports_impl { (fn $ name : ident (& self , $ ($ argn : ident : $ argt : ty) ,*)) => { fn $ name (& self , $ ($ argn : $ argt) ,*) { if self . cli_enabled { self . cli .$ name ($ ($ argn) ,*) ; } if self . bencher_enabled { self . bencher .$ name ($ ($ argn) ,*) ; } # [cfg (feature = "csv_output")] if self . csv_enabled { FileCsvReport .$ name ($ ($ argn) ,*) ; } if let Some (reporter) = & self . html { reporter .$ name ($ ($ argn) ,*) ; } } } ; }
    };
}

reports_impl!();