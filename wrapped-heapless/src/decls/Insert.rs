macro_rules! deps {
    () => {
        Inserted!();
    };
}

macro_rules! Insert {
    () => {
        deps!();
        enum Insert < K , V > { Success (Inserted < V >) , Full ((K , V)) , }
    };
}

Insert!();