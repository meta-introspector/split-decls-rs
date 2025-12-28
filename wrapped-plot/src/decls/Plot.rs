macro_rules! deps {
    () => {
        Matrix!();
    };
}

macro_rules! Plot {
    () => {
        deps!();
        # [derive (Clone)] struct Plot { data : Matrix , script : String , }
    };
}

Plot!();