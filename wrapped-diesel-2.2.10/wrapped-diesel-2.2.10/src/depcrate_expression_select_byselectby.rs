// Generated macro for SelectBy (struct)
macro_rules! Depcrate_expression_select_bySelectBy {
() => {
// Module: crate::expression::select_by
// Provides: {"SelectBy"}
// Dependencies: {}
# [derive (Debug)] pub struct SelectBy < T : Selectable < DB > , DB : Backend > { selection : T :: SelectExpression , p : std :: marker :: PhantomData < (T , DB) > , }
};
}
