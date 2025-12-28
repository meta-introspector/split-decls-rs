macro_rules! deps {
    () => {
        TrackedAssignment!();
    };
}

macro_rules! Assignments {
    () => {
        deps!();
        # [doc = " A typically sized list of attributes."] pub type Assignments = SmallVec < TrackedAssignment , AVERAGE_NUM_ATTRS > ;
    };
}

Assignments!();