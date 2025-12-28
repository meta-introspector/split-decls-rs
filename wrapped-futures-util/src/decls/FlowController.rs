macro_rules! deps {
    () => {
        FlowStep!();
    };
}

macro_rules! FlowController {
    () => {
        deps!();
        # [doc = " Returns the next flow step based on the received item."] pub trait FlowController < I , O > { # [doc = " Handles an item producing `FlowStep` describing the next flow step."] fn next_step (item : I) -> FlowStep < I , O > ; }
    };
}

FlowController!();