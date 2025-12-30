// Generated macro for try_set_handler (function)
macro_rules! Depcratetry_set_handler {
() => {
// Module: crate
// Provides: {"try_set_handler"}
// Dependencies: {}
# [doc = " The same as ctrlc::set_handler but errors if a handler already exists for the signal(s)."] # [doc = ""] # [doc = " # Errors"] # [doc = " Will return an error if another handler exists or if a system error occurred while setting the"] # [doc = " handler."] pub fn try_set_handler < F > (user_handler : F) -> Result < () , Error > where F : FnMut () + 'static + Send , { init_and_set_handler (user_handler , false) }
};
}
