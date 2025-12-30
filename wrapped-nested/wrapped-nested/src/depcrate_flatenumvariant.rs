// Generated macro for EnumVariant (enum)
macro_rules! Depcrate_flatEnumVariant {
() => {
// Module: crate::flat
// Provides: {"EnumVariant"}
// Dependencies: {}
enum EnumVariant < 'sval , S : Stream < 'sval > > { Tagged (Tagged < FlatStreamEnum < S :: Enum > >) , Tuple (Tuple < < S :: Enum as StreamEnum < 'sval > > :: Tuple >) , Record (Record < < S :: Enum as StreamEnum < 'sval > > :: Record >) , }
};
}
