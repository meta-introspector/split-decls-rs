// Generated macro for macro_5 (macro)
macro_rules! Depcratemacro_5 {
() => {
// Module: crate
// Provides: {"macro_5"}
// Dependencies: {}
extern_item ! { { #! [cfg_attr (link_asm , link (name = "psm_s"))] # [cfg (asm)] fn rust_psm_stack_direction () -> u8 ; # [cfg (asm)] fn rust_psm_stack_pointer () -> * mut u8 ; # [cfg (all (switchable_stack , not (target_os = "windows")))] # [link_name = "rust_psm_replace_stack"] fn _rust_psm_replace_stack (data : usize , callback : extern_item ! (unsafe fn (usize) -> !) , sp : * mut u8) -> !; # [cfg (all (switchable_stack , not (target_os = "windows")))] # [link_name = "rust_psm_on_stack"] fn _rust_psm_on_stack (data : usize , return_ptr : usize , callback : extern_item ! (unsafe fn (usize , usize)) , sp : * mut u8 ,) ; # [cfg (all (switchable_stack , target_os = "windows"))] fn rust_psm_replace_stack (data : usize , callback : extern_item ! (unsafe fn (usize) -> !) , sp : * mut u8 , stack_base : * mut u8) -> !; # [cfg (all (switchable_stack , target_os = "windows"))] fn rust_psm_on_stack (data : usize , return_ptr : usize , callback : extern_item ! (unsafe fn (usize , usize)) , sp : * mut u8 , stack_base : * mut u8) ; } }
};
}
