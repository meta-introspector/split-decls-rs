// Generated macro for impls (module)
macro_rules! Depcrate_keyimpls {
() => {
// Module: crate::key
// Provides: {"impls"}
// Dependencies: {}
mod impls { use bstr :: { BStr , BString , ByteSlice } ; use crate :: key :: { AsKey , KeyRef } ; impl AsKey for String { fn as_key (& self) -> KeyRef < '_ > { self . try_as_key () . unwrap_or_else (| | panic ! ("'{self}' is not a valid configuration key")) } fn try_as_key (& self) -> Option < KeyRef < '_ > > { KeyRef :: parse_unvalidated (self . as_str () . into ()) } } impl AsKey for & str { fn as_key (& self) -> KeyRef < '_ > { self . try_as_key () . unwrap_or_else (| | panic ! ("'{self}' is not a valid configuration key")) } fn try_as_key (& self) -> Option < KeyRef < '_ > > { KeyRef :: parse_unvalidated ((* self) . into ()) } } impl AsKey for BString { fn as_key (& self) -> KeyRef < '_ > { self . try_as_key () . unwrap_or_else (| | panic ! ("'{self}' is not a valid configuration key")) } fn try_as_key (& self) -> Option < KeyRef < '_ > > { KeyRef :: parse_unvalidated (self . as_bstr ()) } } impl AsKey for & BStr { fn as_key (& self) -> KeyRef < '_ > { self . try_as_key () . unwrap_or_else (| | panic ! ("'{self}' is not a valid configuration key")) } fn try_as_key (& self) -> Option < KeyRef < '_ > > { KeyRef :: parse_unvalidated (self) } } impl < T > AsKey for & T where T : AsKey , { fn as_key (& self) -> KeyRef < '_ > { (* self) . as_key () } fn try_as_key (& self) -> Option < KeyRef < '_ > > { (* self) . try_as_key () } } impl AsKey for KeyRef < '_ > { fn as_key (& self) -> KeyRef < '_ > { * self } fn try_as_key (& self) -> Option < KeyRef < '_ > > { Some (* self) } } }
};
}
