// Generated macro for impl_58 (impl)
macro_rules! Depcrate_bordersimpl_58 {
() => {
// Module: crate::borders
// Provides: {"impl_58"}
// Dependencies: {}
impl BorderType { # [doc = " Convert this `BorderType` into the corresponding [`Set`](border::Set) of border symbols."] pub const fn border_symbols < 'a > (border_type : Self) -> border :: Set < 'a > { match border_type { Self :: Plain => border :: PLAIN , Self :: Rounded => border :: ROUNDED , Self :: Double => border :: DOUBLE , Self :: Thick => border :: THICK , Self :: LightDoubleDashed => border :: LIGHT_DOUBLE_DASHED , Self :: HeavyDoubleDashed => border :: HEAVY_DOUBLE_DASHED , Self :: LightTripleDashed => border :: LIGHT_TRIPLE_DASHED , Self :: HeavyTripleDashed => border :: HEAVY_TRIPLE_DASHED , Self :: LightQuadrupleDashed => border :: LIGHT_QUADRUPLE_DASHED , Self :: HeavyQuadrupleDashed => border :: HEAVY_QUADRUPLE_DASHED , Self :: QuadrantInside => border :: QUADRANT_INSIDE , Self :: QuadrantOutside => border :: QUADRANT_OUTSIDE , } } # [doc = " Convert this `BorderType` into the corresponding [`Set`](border::Set) of border symbols."] pub const fn to_border_set < 'a > (self) -> border :: Set < 'a > { Self :: border_symbols (self) } }
};
}
