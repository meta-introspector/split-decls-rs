macro_rules! deps {
    () => {
        Step!();
        Duration!();
    };
}

macro_rules! State {
    () => {
        deps!();
        # [derive (Clone , Eq , PartialEq , Ord , PartialOrd , Debug)] struct State { observed : Duration , last_value : progress :: Step , elapsed_values : VecDeque < (Duration , progress :: Step) > , last_update_duration : Duration , precomputed_throughput : Option < progress :: Step > , }
    };
}

State!()