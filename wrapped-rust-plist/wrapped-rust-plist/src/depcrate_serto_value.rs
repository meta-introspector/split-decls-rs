// Generated macro for to_value (function)
macro_rules! Depcrate_serto_value {
() => {
// Module: crate::ser
// Provides: {"to_value"}
// Dependencies: {}
# [doc = " Converts a `T` into a [`Value`] which can represent any valid plist."] pub fn to_value < T : ser :: Serialize > (value : & T) -> Result < Value , Error > { let writer = crate :: value :: Builder :: default () ; let mut ser = Serializer :: new (writer) ; value . serialize (& mut ser) ? ; ser . into_inner () . finish () }
};
}
