// Generated macro for impl_267 (impl)
macro_rules! Depcrate_progress_logimpl_267 {
() => {
// Module: crate::progress::log
// Provides: {"impl_267"}
// Dependencies: {}
impl NestedProgress for Log { type SubProgress = Log ; fn add_child (& mut self , name : impl Into < String >) -> Self :: SubProgress { self . add_child_with_id (name , crate :: progress :: UNKNOWN) } fn add_child_with_id (& mut self , name : impl Into < String > , id : Id) -> Self :: SubProgress { Log { name : format ! ("{}{}{}" , self . name , SEP , Into ::< String >:: into (name)) , id , current_level : self . current_level + 1 , max_level : self . max_level , step : Default :: default () , max : None , unit : None , trigger : Arc :: clone (& self . trigger) , } } }
};
}
