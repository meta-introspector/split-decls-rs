// Generated macro for max_items (function)
macro_rules! Depcrate_validators_max_itemsmax_items {
() => {
// Module: crate::validators::max_items
// Provides: {"max_items"}
// Dependencies: {}
pub fn max_items < T : Deref < Target = [E] > + InputType , E > (value : & T , len : usize ,) -> Result < () , InputValueError < T > > { if value . deref () . len () <= len { Ok (()) } else { Err (format ! ("the value length is {}, must be less than or equal to {}" , value . deref () . len () , len) . into ()) } }
};
}
