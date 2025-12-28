macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! macro_946 {
    () => {
        deps!();
        pin_project ! { # [doc = " Sink for the [`sink_map_err`](super::SinkExt::sink_map_err) method."] # [derive (Debug , Clone)] # [must_use = "sinks do nothing unless polled"] pub struct SinkMapErr < Si , F > { # [pin] sink : Si , f : Option < F >, } }
    };
}

macro_946!();