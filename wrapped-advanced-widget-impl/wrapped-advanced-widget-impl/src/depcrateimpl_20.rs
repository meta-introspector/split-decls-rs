// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl Default for BoxedSquares { fn default () -> Self { let red_square : Box < dyn WidgetRef > = Box :: new (RedSquare) ; let blue_square : Box < dyn WidgetRef > = Box :: new (BlueSquare) ; Self { squares : vec ! [red_square , blue_square] , } } }
};
}
