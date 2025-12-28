macro_rules! deps {
    () => {
        StringId!();
    };
}

macro_rules! EventId {
    () => {
        deps!();
        # [doc = " An `EventId` is a `StringId` with the additional guarantee that the"] # [doc = " corresponding string conforms to the event_id grammar."] # [derive (Clone , Copy , Eq , PartialEq , Hash , Debug)] # [repr (C)] pub struct EventId (StringId) ;
    };
}

EventId!();