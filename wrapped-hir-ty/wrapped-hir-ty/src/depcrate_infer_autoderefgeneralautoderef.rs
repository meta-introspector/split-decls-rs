// Generated macro for GeneralAutoderef (struct)
macro_rules! Depcrate_infer_autoderefGeneralAutoderef {
() => {
// Module: crate::infer::autoderef
// Provides: {"GeneralAutoderef"}
// Dependencies: {}
# [doc = " Recursively dereference a type, considering both built-in"] # [doc = " dereferences (`*`) and the `Deref` trait."] # [doc = " Although called `Autoderef` it can be configured to use the"] # [doc = " `Receiver` trait instead of the `Deref` trait."] pub (crate) struct GeneralAutoderef < 'db , Ctx , Steps = Vec < (Ty < 'db > , AutoderefKind) > > { ctx : Ctx , traits : Option < AutoderefTraits > , state : AutoderefSnapshot < 'db , Steps > , include_raw_pointers : bool , use_receiver_trait : bool , }
};
}
