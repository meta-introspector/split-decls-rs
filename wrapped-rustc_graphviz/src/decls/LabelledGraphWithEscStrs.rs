macro_rules! deps {
    () => {
        LabelledGraph!();
    };
}

macro_rules! LabelledGraphWithEscStrs {
    () => {
        deps!();
        struct LabelledGraphWithEscStrs { graph : LabelledGraph , }
    };
}

LabelledGraphWithEscStrs!();