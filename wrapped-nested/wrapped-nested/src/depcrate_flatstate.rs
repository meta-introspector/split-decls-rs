// Generated macro for State (enum)
macro_rules! Depcrate_flatState {
() => {
// Module: crate::flat
// Provides: {"State"}
// Dependencies: {}
enum State < 'sval , S : Stream < 'sval > > { Any (Option < Any < 'sval , S > >) , Seq (Option < Seq < S :: Seq > >) , Map (Option < Map < S :: Map > >) , Tagged (Option < Tagged < S > >) , Tuple (Option < Tuple < S :: Tuple > >) , Record (Option < Record < S :: Record > >) , Enum (Option < Enum < FlatStreamEnum < S :: Enum > > >) , EnumVariant (Option < EnumVariant < 'sval , S > >) , Done (Option < Result < S :: Ok > >) , }
};
}
