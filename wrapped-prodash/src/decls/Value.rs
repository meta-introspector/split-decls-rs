macro_rules! deps {
    () => {
        State!();
        Progress!();
        Unit!();
        Step!();
        StepShared!();
    };
}

macro_rules! Value {
    () => {
        deps!();
        # [doc = " Progress associated with some item in the progress tree."] # [derive (Clone , Default , Debug)] pub struct Value { # [doc = " The amount of progress currently made"] pub step : StepShared , # [doc = " The step at which no further progress has to be made."] # [doc = ""] # [doc = " If unset, the progress is unbounded."] pub done_at : Option < Step > , # [doc = " The unit associated with the progress."] pub unit : Option < Unit > , # [doc = " Whether progress can be made or not"] pub state : State , }
    };
}

Value!()