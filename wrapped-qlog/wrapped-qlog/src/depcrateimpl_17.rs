// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
# [doc = " Helper functions for using a qlog [Trace]."] impl Trace { # [doc = " Creates a new qlog [Trace]"] pub fn new (vantage_point : VantagePoint , title : Option < String > , description : Option < String > , configuration : Option < Configuration > , common_fields : Option < CommonFields > ,) -> Self { Trace { vantage_point , title , description , configuration , common_fields , events : Vec :: new () , } } # [doc = " Append an [Event] to a [Trace]"] pub fn push_event (& mut self , event : Event) { self . events . push (event) ; } }
};
}
