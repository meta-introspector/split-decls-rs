// Generated macro for Event (enum)
macro_rules! Depcrate_streamEvent {
() => {
// Module: crate::stream
// Provides: {"Event"}
// Dependencies: {}
# [doc = " An encoding of a plist as a flat structure."] # [doc = ""] # [doc = " Output by the event readers."] # [doc = ""] # [doc = " Dictionary keys and values are represented as pairs of values e.g.:"] # [doc = ""] # [doc = " ```ignore rust"] # [doc = " StartDictionary"] # [doc = " String(\"Height\") // Key"] # [doc = " Real(181.2)      // Value"] # [doc = " String(\"Age\")    // Key"] # [doc = " Integer(28)      // Value"] # [doc = " EndDictionary"] # [doc = " ```"] # [doc = ""] # [doc = " ## Lifetimes"] # [doc = ""] # [doc = " This type has a lifetime parameter; during serialization, data is borrowed"] # [doc = " from a [`Value`], and the lifetime of the event is the lifetime of the"] # [doc = " [`Value`] being serialized."] # [doc = ""] # [doc = " During deserialization, data is always copied anyway, and this lifetime"] # [doc = " is always `'static`."] # [derive (Clone , Debug , PartialEq)] # [non_exhaustive] pub enum Event < 'a > { StartArray (Option < u64 >) , StartDictionary (Option < u64 >) , EndCollection , Boolean (bool) , Data (Cow < 'a , [u8] >) , Date (Date) , Integer (Integer) , Real (f64) , String (Cow < 'a , str >) , Uid (Uid) , }
};
}
