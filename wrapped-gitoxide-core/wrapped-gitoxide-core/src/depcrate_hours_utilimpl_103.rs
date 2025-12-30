// Generated macro for impl_103 (impl)
macro_rules! Depcrate_hours_utilimpl_103 {
() => {
// Module: crate::hours::util
// Provides: {"impl_103"}
// Dependencies: {}
impl WorkByPerson { pub fn merge (& mut self , other : & WorkByEmail) { if ! self . name . contains (& other . name) { self . name . push (other . name) ; } if ! self . email . contains (& other . email) { self . email . push (other . email) ; } self . num_commits += other . num_commits ; self . hours += other . hours ; self . files . add (& other . files) ; self . lines . add (& other . lines) ; } }
};
}
