macro_rules! deps {
    () => {
        TimingGuard!();
        EventId!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < 'a > TimingGuard < 'a > { # [doc = " This method set a new `event_id` right before actually recording the"] # [doc = " event."] # [inline] pub fn finish_with_override_event_id (mut self , event_id : EventId) { self . event_id = event_id ; drop (self) } }
    };
}

impl_41!()