// Generated macro for run (function)
macro_rules! Depcrate_transforms_multi_valuerun {
() => {
// Module: crate::transforms::multi_value
// Provides: {"run"}
// Dependencies: {}
# [doc = " Run the transformation."] # [doc = ""] # [doc = " See the module-level docs for details on the transformation."] # [doc = ""] # [doc = " * `memory` is the module's memory that has the stack where return"] # [doc = "   pointers are allocated within."] # [doc = ""] # [doc = " * `__stack_pointer` is the global that is being used as the stack"] # [doc = "   pointer. With LLVM, this is typically the first global."] # [doc = ""] # [doc = " * `to_xform` is the set of exported functions we want to transform and"] # [doc = "   information required to transform them. The `usize` is the index of the"] # [doc = "   return pointer parameter that will be removed. The `Vec<walrus::ValType>`"] # [doc = "   is the new result type that will be returned directly instead of via the"] # [doc = "   return pointer."] # [doc = ""] # [doc = " Returns a list of wrappers which have multi value signatures and call the"] # [doc = " corresponding element in the `to_xform` list."] pub fn run (module : & mut walrus :: Module , memory : walrus :: MemoryId , stack_pointer : walrus :: GlobalId , to_xform : & [(walrus :: FunctionId , usize , Vec < walrus :: ValType >)] ,) -> Result < Vec < walrus :: FunctionId > , anyhow :: Error > { crate :: wasm_conventions :: insert_target_feature (module , "multivalue") . context ("failed to parse `target_features` custom section") ? ; let mut wrappers = Vec :: new () ; for (func , return_pointer_index , results) in to_xform { wrappers . push (xform_one (module , memory , stack_pointer , * func , * return_pointer_index , results ,) ?) ; } Ok (wrappers) }
};
}
