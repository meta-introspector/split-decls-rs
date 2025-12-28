macro_rules! deps {
    () => {
        EventId!();
        StringId!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl EventId { pub const INVALID : EventId = EventId (StringId :: INVALID) ; # [inline] pub fn to_string_id (self) -> StringId { self . 0 } # [inline] pub fn as_u64 (self) -> u64 { self . 0 . as_u64 () } # [inline] pub fn from_label (label : StringId) -> Self { EventId (label) } # [inline] pub fn from_virtual (virtual_id : StringId) -> Self { EventId (virtual_id) } # [doc = " Create an EventId from a raw u64 value. Only used internally for"] # [doc = " deserialization."] # [inline] pub fn from_u64 (raw_id : u64) -> Self { EventId (StringId :: new (raw_id)) } }
    };
}

impl_20!()