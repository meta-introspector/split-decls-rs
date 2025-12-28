macro_rules! deps {
    () => {
        EntryMarker!();
        CompletionQueue!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < E : EntryMarker > Drop for CompletionQueue < '_ , E > { # [inline] fn drop (& mut self) { unsafe { & * self . queue . head } . store (self . head , atomic :: Ordering :: Release) ; } }
    };
}

impl_19!()