// Generated macro for impl_197 (impl)
macro_rules! Depcrate_executor_look_aheadimpl_197 {
() => {
// Module: crate::executor::look_ahead
// Provides: {"impl_197"}
// Dependencies: {}
impl < 'a , S : ScalarValue + 'a > LookAheadValue < 'a , S > { fn from_input_value (input_value : BorrowedSpanning < 'a , & 'a InputValue < S > > , vars : Option < & 'a Variables < S > > ,) -> BorrowedSpanning < 'a , Self > { let Spanning { item : input_value , span : input_span , } = input_value ; Spanning { item : match input_value { InputValue :: Null => Self :: Null , InputValue :: Scalar (s) => Self :: Scalar (s) , InputValue :: Enum (e) => Self :: Enum (e) , InputValue :: Variable (name) => vars . and_then (| vars | vars . get (name)) . map (| item | { Self :: from_input_value (BorrowedSpanning { item , span : input_span , } , vars ,) . item }) . unwrap_or (Self :: Null) , InputValue :: List (input_list) => Self :: List (LookAheadList { input_list , vars }) , InputValue :: Object (input_object) => Self :: Object (LookAheadObject { input_object : input_object . as_slice () , vars , }) , } , span : input_span , } } }
};
}
