// Generated macro for impl_1242 (impl)
macro_rules! Depcrate_runtime_protocol_objectimpl_1242 {
() => {
// Module: crate::runtime::protocol_object
// Provides: {"impl_1242"}
// Dependencies: {}
impl < P : ? Sized > ProtocolObject < P > { # [doc = " Get an immutable type-erased reference from a type implementing a"] # [doc = " protocol."] # [inline] pub fn from_ref < T : ? Sized + Message > (obj : & T) -> & Self where P : ImplementedBy < T > , { let ptr : NonNull < T > = NonNull :: from (obj) ; let ptr : NonNull < Self > = ptr . cast () ; unsafe { ptr . as_ref () } } # [doc = " Get a type-erased object from a type implementing a protocol."] # [deprecated = "use `ProtocolObject::from_retained` instead"] # [inline] pub fn from_id < T > (obj : Retained < T >) -> Retained < Self > where P : ImplementedBy < T > + 'static , T : Message + 'static , { Self :: from_retained (obj) } # [doc = " Get a type-erased object from a type implementing a protocol."] # [inline] pub fn from_retained < T > (obj : Retained < T >) -> Retained < Self > where P : ImplementedBy < T > + 'static , T : Message + 'static , { unsafe { Retained :: cast_unchecked :: < Self > (obj) } } }
};
}
