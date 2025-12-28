macro_rules! deps {
    () => {
        Statistic!();
        Slope!();
    };
}

macro_rules! REPORT_STATS {
    () => {
        deps!();
        const REPORT_STATS : [Statistic ; 7] = [Statistic :: Typical , Statistic :: Slope , Statistic :: Mean , Statistic :: Median , Statistic :: MedianAbsDev , Statistic :: MedianAbsDev , Statistic :: StdDev ,] ;
    };
}

REPORT_STATS!();