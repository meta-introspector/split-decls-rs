// Generated macro for get_name (function)
macro_rules! Depcrate_decorators_inlineget_name {
() => {
// Module: crate::decorators::inline
// Provides: {"get_name"}
// Dependencies: {}
fn get_name < 'reg : 'rc , 'rc > (d : & Decorator < 'rc >) -> Result < String , RenderError > { d . param (0) . ok_or_else (| | RenderErrorReason :: ParamNotFoundForIndex ("inline" , 0) . into ()) . and_then (| v | { v . value () . as_str () . map (std :: borrow :: ToOwned :: to_owned) . ok_or_else (| | RenderErrorReason :: InvalidParamType ("String") . into ()) }) }
};
}
