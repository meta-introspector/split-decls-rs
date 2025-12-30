// Generated macro for Cast (struct)
macro_rules! Depcrate_refs_castCast {
() => {
// Module: crate::refs::cast
// Provides: {"Cast"}
// Dependencies: {}
# [doc = " Represents a runtime checked (via `IsInstanceOf`) cast of a reference from one type to another"] # [doc = ""] # [doc = " This borrows a reference and implements `Deref` for the target type."] # [doc = ""] # [doc = " This can be used to cast global or local references."] # [doc = ""] # [doc = " See: [Env::as_cast]"] # [doc = ""] # [repr (transparent)] # [derive (Debug)] pub struct Cast < 'any , 'from , To : Reference > { _from : PhantomData < & 'from JObject < 'any > > , to : To :: Kind < 'any > , }
};
}
