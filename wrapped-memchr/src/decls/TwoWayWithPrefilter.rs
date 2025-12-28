macro_rules! deps {
    () => {
        Finder!();
        Prefilter!();
    };
}

macro_rules! TwoWayWithPrefilter {
    () => {
        deps!();
        # [doc = " A two-way substring searcher with a prefilter."] # [derive (Copy , Clone , Debug)] struct TwoWayWithPrefilter { finder : twoway :: Finder , prestrat : Prefilter , }
    };
}

TwoWayWithPrefilter!();