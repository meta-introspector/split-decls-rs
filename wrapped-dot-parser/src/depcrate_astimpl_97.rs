// Generated macro for impl_97 (impl)
macro_rules! Depcrate_astimpl_97 {
() => {
// Module: crate::ast
// Provides: {"impl_97"}
// Dependencies: {}
# [cfg (feature = "to_tokens")] impl ToTokens for CompassPt { fn to_tokens (& self , tokens : & mut TokenStream) { let new_tokens = match self { CompassPt :: N => quote ! { dot_parser :: ast :: CompassPt :: N } , CompassPt :: NE => quote ! { dot_parser :: ast :: CompassPt :: NE } , CompassPt :: E => quote ! { dot_parser :: ast :: CompassPt :: E } , CompassPt :: SE => quote ! { dot_parser :: ast :: CompassPt :: SE } , CompassPt :: S => quote ! { dot_parser :: ast :: CompassPt :: S } , CompassPt :: SW => quote ! { dot_parser :: ast :: CompassPt :: SW } , CompassPt :: W => quote ! { dot_parser :: ast :: CompassPt :: W } , CompassPt :: NW => quote ! { dot_parser :: ast :: CompassPt :: NW } , CompassPt :: C => quote ! { dot_parser :: ast :: CompassPt :: C } , CompassPt :: Underscore => quote ! { dot_parser :: ast :: CompassPt :: Underscore } , } ; tokens . append_all (new_tokens) ; } }
};
}
