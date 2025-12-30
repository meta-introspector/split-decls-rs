// Generated macro for inject_destroy (function)
macro_rules! Depcrate_transforms_threadsinject_destroy {
() => {
// Module: crate::transforms::threads
// Provides: {"inject_destroy"}
// Dependencies: {}
fn inject_destroy (module : & mut Module , tls : & Tls , stack : & Stack , memory : MemoryId ,) -> Result < () , Error > { let free = find_function (module , "__wbindgen_free") ? ; let mut builder = FunctionBuilder :: new (& mut module . types , & [ValType :: I32 , ValType :: I32 , ValType :: I32] , & [] ,) ; builder . name ("__wbindgen_thread_destroy" . into ()) ; let mut body = builder . func_body () ; let tls_base = module . locals . add (ValType :: I32) ; let stack_alloc = module . locals . add (ValType :: I32) ; let stack_size = module . locals . add (ValType :: I32) ; body . local_get (tls_base) . if_else (None , | body | { body . local_get (tls_base) . i32_const (tls . size as i32) . i32_const (tls . align as i32) . call (free) ; } , | body | { body . global_get (tls . base) . i32_const (tls . size as i32) . i32_const (tls . align as i32) . call (free) ; body . i32_const (i32 :: MIN) . global_set (tls . base) ; } ,) ; body . local_get (stack_alloc) . if_else (None , | body | { body . local_get (stack_alloc) . local_get (stack_size) . i32_const (DEFAULT_THREAD_STACK_SIZE as i32) . local_get (stack_size) . select (None) . i32_const (16) . call (free) ; } , | body | { with_temp_stack (body , memory , stack , | body | { body . global_get (stack . alloc) . global_get (stack . size) . i32_const (16) . call (free) ; }) ; body . i32_const (0) . global_set (stack . alloc) ; } ,) ; let destroy_id = builder . finish (vec ! [tls_base , stack_alloc , stack_size] , & mut module . funcs) ; module . exports . add ("__wbindgen_thread_destroy" , destroy_id) ; Ok (()) }
};
}
