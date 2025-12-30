// Generated macro for FunctionBindgen (struct)
macro_rules! Depcrate_functionFunctionBindgen {
() => {
// Module: crate::function
// Provides: {"FunctionBindgen"}
// Dependencies: {}
# [doc = " FunctionBindgen generates the C# code for calling functions defined in wit"] pub (crate) struct FunctionBindgen < 'a , 'b > { pub (crate) interface_gen : & 'b mut InterfaceGenerator < 'a > , func_name : & 'b str , kind : & 'b FunctionKind , params : Box < [String] > , results : Vec < TypeId > , pub (crate) src : String , locals : Ns , block_storage : Vec < BlockStorage > , blocks : Vec < Block > , payloads : Vec < String > , pub (crate) needs_cleanup : bool , import_return_pointer_area_size : usize , import_return_pointer_area_align : usize , pub (crate) resource_drops : Vec < (String , String) > , is_block : bool , fixed_statments : Vec < Fixed > , parameter_type : ParameterType , result_type : Option < Type > , }
};
}
