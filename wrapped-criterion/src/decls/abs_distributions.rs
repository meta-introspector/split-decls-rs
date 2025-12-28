macro_rules! deps {
    () => {
        ValueFormatter!();
        MeasurementData!();
        ReportContext!();
        BenchmarkId!();
    };
}

macro_rules! abs_distributions {
    () => {
        deps!();
        pub (crate) fn abs_distributions (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , size : Option < (u32 , u32) > ,) { crate :: plot :: REPORT_STATS . iter () . filter_map (| stat | { measurements . distributions . get (* stat) . and_then (| dist | { measurements . absolute_estimates . get (* stat) . map (| est | (* stat , dist , est)) }) }) . for_each (| (statistic , distribution , estimate) | { abs_distribution (id , context , formatter , statistic , distribution , estimate , size ,) ; }) ; }
    };
}

abs_distributions!()