// Generated macro for min_items (function)
macro_rules! Depcrate_validators_min_itemsmin_items {
() => {
// Module: crate::validators::min_items
// Provides: {"min_items"}
// Dependencies: {}
pub fn min_items < T : Deref < Target = [E] > + InputType , E > (value : & T , len : usize ,) -> Result < () , InputValueError < T > > { if value . deref () . len () >= len { Ok (()) } else { Err (format ! ("the value length is {}, must be greater than or equal to {}" , value . deref () . len () , len) . into ()) } }
};
}
