macro_rules! deps {
    () => {
        RawEvent!();
        EventId!();
        StringId!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl Default for RawEvent { fn default () -> Self { RawEvent { event_kind : StringId :: INVALID , event_id : EventId :: INVALID , thread_id : 0 , payload1_lower : 0 , payload2_lower : 0 , payloads_upper : 0 , } } }
    };
}

impl_50!()