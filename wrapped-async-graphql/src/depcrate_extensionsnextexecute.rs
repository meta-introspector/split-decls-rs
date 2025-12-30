// Generated macro for NextExecute (struct)
macro_rules! Depcrate_extensionsNextExecute {
() => {
// Module: crate::extensions
// Provides: {"NextExecute"}
// Dependencies: {}
# [doc = " The remainder of a extension chain for execute."] pub struct NextExecute < 'a > { chain : & 'a [Arc < dyn Extension >] , execute_fut_factory : ExecuteFutFactory < 'a > , execute_data : Option < Data > , }
};
}
