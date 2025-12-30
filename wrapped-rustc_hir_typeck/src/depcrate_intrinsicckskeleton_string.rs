// Generated macro for skeleton_string (function)
macro_rules! Depcrate_intrinsicckskeleton_string {
() => {
// Module: crate::intrinsicck
// Provides: {"skeleton_string"}
// Dependencies: {}
# [doc = " Try to display a sensible error with as much information as possible."] fn skeleton_string < 'tcx > (ty : Ty < 'tcx > , sk : Result < SizeSkeleton < 'tcx > , & 'tcx LayoutError < 'tcx > > ,) -> String { match sk { Ok (SizeSkeleton :: Pointer { tail , .. }) => format ! ("pointer to `{tail}`") , Ok (SizeSkeleton :: Known (size , _)) => { if let Some (v) = u128 :: from (size . bytes ()) . checked_mul (8) { format ! ("{v} bits") } else { bug ! ("{:?} overflow for u128" , size) } } Ok (SizeSkeleton :: Generic (size)) => { format ! ("generic size {size}") } Err (LayoutError :: TooGeneric (bad)) => { if * bad == ty { "this type does not have a fixed size" . to_owned () } else { format ! ("size can vary because of {bad}") } } Err (err) => err . to_string () , } }
};
}
