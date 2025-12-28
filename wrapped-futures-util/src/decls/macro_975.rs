macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! macro_975 {
    () => {
        deps!();
        pin_project ! { # [doc = " Sink for the [`with_flat_map`](super::SinkExt::with_flat_map) method."] # [must_use = "sinks do nothing unless polled"] pub struct WithFlatMap < Si , Item , U , St , F > { # [pin] sink : Si , f : F , # [pin] stream : Option < St >, buffer : Option < Item >, _marker : PhantomData < fn (U) >, } }
    };
}

macro_975!();