// Generated macro for DateTime (struct)
macro_rules! Depcrate_datetimeDateTime {
() => {
// Module: crate::datetime
// Provides: {"DateTime"}
// Dependencies: {}
# [doc = " ISO 8601 combined date and time with time zone."] # [doc = ""] # [doc = " There are some constructors implemented here (the `from_*` methods), but"] # [doc = " the general-purpose constructors are all via the methods on the"] # [doc = " [`TimeZone`](./offset/trait.TimeZone.html) implementations."] # [derive (Clone)] # [cfg_attr (any (feature = "rkyv" , feature = "rkyv-16" , feature = "rkyv-32" , feature = "rkyv-64") , derive (Archive , Deserialize , Serialize) , archive (compare (PartialEq , PartialOrd)))] # [cfg_attr (feature = "rkyv-validation" , archive (check_bytes))] pub struct DateTime < Tz : TimeZone > { datetime : NaiveDateTime , offset : Tz :: Offset , }
};
}
