// Generated macro for impl_130 (impl)
macro_rules! Depcrateimpl_130 {
() => {
// Module: crate
// Provides: {"impl_130"}
// Dependencies: {}
impl Parallelism { pub (crate) fn spawn < OP > (& self , op : OP) where OP : FnOnce () + Send + 'static , { match self { Parallelism :: Serial => op () , Parallelism :: RayonDefaultPool { .. } => rayon :: spawn (op) , Parallelism :: RayonNewPool (num_threads) => { let mut thread_pool = ThreadPoolBuilder :: new () ; if * num_threads > 0 { thread_pool = thread_pool . num_threads (* num_threads) ; } if let Ok (thread_pool) = thread_pool . build () { thread_pool . spawn (op) ; } else { rayon :: spawn (op) ; } } Parallelism :: RayonExistingPool { pool , .. } => pool . spawn (op) , } } pub (crate) fn timeout (& self) -> Option < std :: time :: Duration > { match self { Parallelism :: Serial | Parallelism :: RayonNewPool (_) => None , Parallelism :: RayonDefaultPool { busy_timeout } => Some (* busy_timeout) , Parallelism :: RayonExistingPool { busy_timeout , .. } => * busy_timeout , } } }
};
}
