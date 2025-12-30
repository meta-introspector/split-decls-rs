// Generated macro for Transform (struct)
macro_rules! Depcrate_transforms_externrefTransform {
() => {
// Module: crate::transforms::externref
// Provides: {"Transform"}
// Dependencies: {}
struct Transform < 'a > { cx : & 'a mut Context , intrinsic_map : HashMap < FunctionId , Intrinsic > , import_map : HashMap < FunctionId , FunctionId > , shims : HashSet < FunctionId > , table : TableId , clone_ref : Option < FunctionId > , heap_alloc : Option < FunctionId > , heap_dealloc : Option < FunctionId > , stack_pointer : GlobalId , }
};
}
