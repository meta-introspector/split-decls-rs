// Generated macro for Priority (struct)
macro_rules! Depcrate_h3Priority {
() => {
// Module: crate::h3
// Provides: {"Priority"}
// Dependencies: {}
# [doc = " Extensible Priorities parameters."] # [doc = ""] # [doc = " The `TryFrom` trait supports constructing this object from the serialized"] # [doc = " Structured Fields Dictionary field value. I.e, use `TryFrom` to parse the"] # [doc = " value of a Priority header field or a PRIORITY_UPDATE frame. Using this"] # [doc = " trait requires the `sfv` feature to be enabled."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (C)] pub struct Priority { urgency : u8 , incremental : bool , }
};
}
