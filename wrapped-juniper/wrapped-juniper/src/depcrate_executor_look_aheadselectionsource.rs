// Generated macro for SelectionSource (enum)
macro_rules! Depcrate_executor_look_aheadSelectionSource {
() => {
// Module: crate::executor::look_ahead
// Provides: {"SelectionSource"}
// Dependencies: {}
# [derive (Debug)] pub (super) enum SelectionSource < 'a , S > { Field (& 'a Field < 'a , S >) , Spread { field_name : & 'a str , set : Option < & 'a [Selection < 'a , S >] > , } , }
};
}
