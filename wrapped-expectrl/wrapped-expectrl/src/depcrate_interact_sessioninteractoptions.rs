// Generated macro for InteractOptions (struct)
macro_rules! Depcrate_interact_sessionInteractOptions {
() => {
// Module: crate::interact::session
// Provides: {"InteractOptions"}
// Dependencies: {}
# [doc = " Interact options (aka callbacks you can set to be callled being in an interactive mode)."] struct InteractOptions < S , I , O , C > { state : C , input_filter : Option < OptFilter > , output_filter : Option < OptFilter > , input_action : Option < OptAction < S , I , O , C > > , output_action : Option < OptAction < S , I , O , C > > , idle_action : Option < OptAction < S , I , O , C > > , }
};
}
