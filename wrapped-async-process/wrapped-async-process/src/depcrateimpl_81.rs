// Generated macro for impl_81 (impl)
macro_rules! Depcrateimpl_81 {
() => {
// Module: crate
// Provides: {"impl_81"}
// Dependencies: {}
impl fmt :: Debug for Command { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if f . alternate () { f . debug_struct ("Command") . field ("inner" , & self . inner) . field ("stdin" , & self . stdin) . field ("stdout" , & self . stdout) . field ("stderr" , & self . stderr) . field ("reap_on_drop" , & self . reap_on_drop) . field ("kill_on_drop" , & self . kill_on_drop) . finish () } else { fmt :: Debug :: fmt (& self . inner , f) } } }
};
}
