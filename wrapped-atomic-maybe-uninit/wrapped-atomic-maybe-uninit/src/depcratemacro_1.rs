// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
# [cfg (not (any (target_pointer_width = "16" , target_pointer_width = "32" , target_pointer_width = "64" ,)))] compile_error ! ("atomic-maybe-uninit currently only supports targets with {16,32,64}-bit pointer width; \
     if you need support for others, \
     please submit an issue at <https://github.com/taiki-e/atomic-maybe-uninit>") ;
};
}
