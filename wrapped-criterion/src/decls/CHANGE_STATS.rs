macro_rules! deps {
    () => {
        Statistic!();
    };
}

macro_rules! CHANGE_STATS {
    () => {
        deps!();
        const CHANGE_STATS : [Statistic ; 2] = [Statistic :: Mean , Statistic :: Median] ;
    };
}

CHANGE_STATS!()