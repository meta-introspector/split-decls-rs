// Generated macro for is_phantom_data (function)
macro_rules! Depcrate_utilis_phantom_data {
() => {
// Module: crate::util
// Provides: {"is_phantom_data"}
// Dependencies: {}
# [doc = " Returns true iff the given type is of the form `PhantomData<TY>` where"] # [doc = " `TY` can be substituted for any type, including type variables."] pub fn is_phantom_data (path : & syn :: Path) -> bool { let segs = & path . segments ; if segs . is_empty () { return false ; } let mut path = path . clone () ; let lseg = path . segments . pop () . unwrap () . into_value () ; & lseg . ident == "PhantomData" && pseg_has_single_tyvar (& lseg) && match_pathsegs (& path , & ["" , "marker" , "std::marker" , "core::marker" , "::std::marker" , "::core::marker" ,] ,) }
};
}
