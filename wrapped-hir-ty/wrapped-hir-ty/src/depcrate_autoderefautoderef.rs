// Generated macro for Autoderef (struct)
macro_rules! Depcrate_autoderefAutoderef {
() => {
// Module: crate::autoderef
// Provides: {"Autoderef"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct Autoderef < 'table , 'db , T = Vec < (AutoderefKind , Ty) > > { pub (crate) table : & 'table mut InferenceTable < 'db > , ty : Ty , at_start : bool , steps : T , explicit : bool , use_receiver_trait : bool , }
};
}
