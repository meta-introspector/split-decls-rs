macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! macro_940 {
    () => {
        deps!();
        pin_project ! { # [doc = " Sink for the [`sink_err_into`](super::SinkExt::sink_err_into) method."] # [derive (Debug)] # [must_use = "sinks do nothing unless polled"] pub struct SinkErrInto < Si : Sink < Item >, Item , E > { # [pin] sink : SinkMapErr < Si , fn (Si :: Error) -> E >, } }
    };
}

macro_940!()