// Generated macro for impl_461 (impl)
macro_rules! Depcrateimpl_461 {
() => {
// Module: crate
// Provides: {"impl_461"}
// Dependencies: {}
impl < S > fmt :: Debug for ThreadPoolBuilder < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let ThreadPoolBuilder { ref num_threads , ref use_current_thread , ref get_thread_name , ref panic_handler , ref stack_size , ref start_handler , ref exit_handler , spawn_handler : _ , ref breadth_first , } = * self ; struct ClosurePlaceholder ; impl fmt :: Debug for ClosurePlaceholder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("<closure>") } } let get_thread_name = get_thread_name . as_ref () . map (| _ | ClosurePlaceholder) ; let panic_handler = panic_handler . as_ref () . map (| _ | ClosurePlaceholder) ; let start_handler = start_handler . as_ref () . map (| _ | ClosurePlaceholder) ; let exit_handler = exit_handler . as_ref () . map (| _ | ClosurePlaceholder) ; f . debug_struct ("ThreadPoolBuilder") . field ("num_threads" , num_threads) . field ("use_current_thread" , use_current_thread) . field ("get_thread_name" , & get_thread_name) . field ("panic_handler" , & panic_handler) . field ("stack_size" , & stack_size) . field ("start_handler" , & start_handler) . field ("exit_handler" , & exit_handler) . field ("breadth_first" , & breadth_first) . finish () } }
};
}
