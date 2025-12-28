macro_rules! deps {
    () => {
        ComparisonData!();
        BenchmarkId!();
        ReportContext!();
        MeasurementData!();
    };
}

macro_rules! rel_distributions {
    () => {
        deps!();
        pub (crate) fn rel_distributions (id : & BenchmarkId , context : & ReportContext , _measurements : & MeasurementData < '_ > , comparison : & ComparisonData , size : Option < (u32 , u32) > ,) { crate :: plot :: CHANGE_STATS . iter () . for_each (| & statistic | { rel_distribution (id , context , statistic , comparison . relative_distributions . get (statistic) , comparison . relative_estimates . get (statistic) , comparison . noise_threshold , size ,) ; }) ; }
    };
}

rel_distributions!();