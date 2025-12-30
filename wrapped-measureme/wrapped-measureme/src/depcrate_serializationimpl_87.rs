// Generated macro for impl_87 (impl)
macro_rules! Depcrate_serializationimpl_87 {
() => {
// Module: crate::serialization
// Provides: {"impl_87"}
// Dependencies: {}
impl std :: convert :: TryFrom < u8 > for PageTag { type Error = String ; fn try_from (value : u8) -> Result < Self , Self :: Error > { match value { 0 => Ok (PageTag :: Events) , 1 => Ok (PageTag :: StringData) , 2 => Ok (PageTag :: StringIndex) , _ => Err (format ! ("Could not convert byte `{}` to PageTag." , value)) , } } }
};
}
