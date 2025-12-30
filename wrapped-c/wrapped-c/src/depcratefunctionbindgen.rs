// Generated macro for FunctionBindgen (struct)
macro_rules! DepcrateFunctionBindgen {
() => {
// Module: crate
// Provides: {"FunctionBindgen"}
// Dependencies: {}
struct FunctionBindgen < 'a , 'b > { r#gen : & 'a mut InterfaceGenerator < 'b > , locals : Ns , src : wit_bindgen_core :: Source , sig : CSig , func_to_call : & 'a str , block_storage : Vec < wit_bindgen_core :: Source > , blocks : Vec < (String , Vec < String >) > , payloads : Vec < String > , params : Vec < String > , wasm_return : Option < String > , ret_store_cnt : usize , import_return_pointer_area_size : ArchitectureSize , import_return_pointer_area_align : Alignment , # [doc = " State of what to generate for the `task.return` intrinsic in the case"] # [doc = " that this bindings generator is being used for an async export."] # [doc = ""] # [doc = " This typically stays at `DeferredTaskReturn::None` except for the case"] # [doc = " of async exports where they'll fill this in after the `CallInterface`"] # [doc = " instruction. For some more information see the documentation on"] # [doc = " `DeferredTaskReturn`."] deferred_task_return : DeferredTaskReturn , # [doc = " Borrows observed during lifting an export, that will need to be dropped when the guest"] # [doc = " function exits."] borrows : Vec < DroppableBorrow > , # [doc = " Forward declarations for temporary storage of borrow copies."] borrow_decls : wit_bindgen_core :: Source , }
};
}
