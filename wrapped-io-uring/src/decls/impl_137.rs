macro_rules! deps {
    () => {
        EntryMarker!();
        SubmissionQueue!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < E : EntryMarker > Drop for SubmissionQueue < '_ , E > { # [inline] fn drop (& mut self) { unsafe { & * self . queue . tail } . store (self . tail , atomic :: Ordering :: Release) ; } }
    };
}

impl_137!();