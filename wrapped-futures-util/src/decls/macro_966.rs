macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! macro_966 {
    () => {
        deps!();
        pin_project ! { # [doc = " Sink for the [`with`](super::SinkExt::with) method."] # [must_use = "sinks do nothing unless polled"] pub struct With < Si , Item , U , Fut , F > { # [pin] sink : Si , f : F , # [pin] state : Option < Fut >, _phantom : PhantomData < fn (U) -> Item >, } }
    };
}

macro_966!();