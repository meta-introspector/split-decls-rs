macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! TicLabels {
    () => {
        deps!();
        # [doc = " Labels attached to the tics of an axis"] pub struct TicLabels < P , L > { # [doc = " Labels to attach to the tics"] pub labels : L , # [doc = " Position of the tics on the axis"] pub positions : P , }
    };
}

TicLabels!();