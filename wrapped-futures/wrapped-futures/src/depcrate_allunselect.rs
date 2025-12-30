// Generated macro for unselect (function)
macro_rules! Depcrate_allunselect {
() => {
// Module: crate::all
// Provides: {"unselect"}
// Dependencies: {}
fn unselect < T , U , E > (r : Result < (T , U) , (E , U) >) -> Result < T , E > { match r { Ok ((t , _)) => Ok (t) , Err ((e , _)) => Err (e) , } }
};
}
