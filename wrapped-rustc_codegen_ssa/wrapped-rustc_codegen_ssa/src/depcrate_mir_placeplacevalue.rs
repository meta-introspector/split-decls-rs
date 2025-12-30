// Generated macro for PlaceValue (struct)
macro_rules! Depcrate_mir_placePlaceValue {
() => {
// Module: crate::mir::place
// Provides: {"PlaceValue"}
// Dependencies: {}
# [doc = " The location and extra runtime properties of the place."] # [doc = ""] # [doc = " Typically found in a [`PlaceRef`] or an [`OperandValue::Ref`]."] # [doc = ""] # [doc = " As a location in memory, this has no specific type. If you want to"] # [doc = " load or store it using a typed operation, use [`Self::with_type`]."] # [derive (Copy , Clone , Debug)] pub struct PlaceValue < V > { # [doc = " A pointer to the contents of the place."] pub llval : V , # [doc = " This place's extra data if it is unsized, or `None` if null."] pub llextra : Option < V > , # [doc = " The alignment we know for this place."] pub align : Align , }
};
}
