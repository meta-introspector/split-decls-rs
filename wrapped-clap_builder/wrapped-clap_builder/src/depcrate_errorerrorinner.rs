// Generated macro for ErrorInner (struct)
macro_rules! Depcrate_errorErrorInner {
() => {
// Module: crate::error
// Provides: {"ErrorInner"}
// Dependencies: {}
# [derive (Debug)] struct ErrorInner { kind : ErrorKind , # [cfg (feature = "error-context")] context : FlatMap < ContextKind , ContextValue > , message : Option < Message > , source : Option < Box < dyn error :: Error + Send + Sync > > , help_flag : Option < Cow < 'static , str > > , styles : Styles , color_when : ColorChoice , color_help_when : ColorChoice , backtrace : Option < Backtrace > , }
};
}
