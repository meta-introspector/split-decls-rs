macro_rules! deps {
    () => {
        ValueType!();
        PlotContext!();
        PlotData!();
        ValueFormatter!();
        BenchmarkId!();
    };
}

macro_rules! Plotter {
    () => {
        deps!();
        pub (crate) trait Plotter { fn pdf (& mut self , ctx : PlotContext < '_ > , data : PlotData < '_ >) ; fn regression (& mut self , ctx : PlotContext < '_ > , data : PlotData < '_ >) ; fn iteration_times (& mut self , ctx : PlotContext < '_ > , data : PlotData < '_ >) ; fn abs_distributions (& mut self , ctx : PlotContext < '_ > , data : PlotData < '_ >) ; fn rel_distributions (& mut self , ctx : PlotContext < '_ > , data : PlotData < '_ >) ; fn line_comparison (& mut self , ctx : PlotContext < '_ > , formatter : & dyn ValueFormatter , all_curves : & [& (& BenchmarkId , Vec < f64 >)] , value_type : ValueType ,) ; fn violin (& mut self , ctx : PlotContext < '_ > , formatter : & dyn ValueFormatter , all_curves : & [& (& BenchmarkId , Vec < f64 >)] ,) ; fn t_test (& mut self , ctx : PlotContext < '_ > , data : PlotData < '_ >) ; fn wait (& mut self) ; }
    };
}

Plotter!();