// Generated macro for new (function)
macro_rules! Depcrate_teenew {
() => {
// Module: crate::tee
// Provides: {"new"}
// Dependencies: {}
pub fn new < I > (iter : I) -> (Tee < I > , Tee < I >) where I : Iterator , { let buffer = TeeBuffer { backlog : VecDeque :: new () , iter , owner : false , } ; let t1 = Tee { rcbuffer : Rc :: new (RefCell :: new (buffer)) , id : true , } ; let t2 = Tee { rcbuffer : t1 . rcbuffer . clone () , id : false , } ; (t1 , t2) }
};
}
