// Generated macro for get_function_table_entry (function)
macro_rules! Depcrate_wasm_conventionsget_function_table_entry {
() => {
// Module: crate::wasm_conventions
// Provides: {"get_function_table_entry"}
// Dependencies: {}
# [doc = " Looks up a function table entry by index in the main function table."] pub fn get_function_table_entry (module : & Module , idx : u32) -> Result < FunctionId > { let table = module . tables . main_function_table () ? . ok_or_else (| | anyhow ! ("no function table found in module")) ? ; let table = module . tables . get (table) ; for & segment in table . elem_segments . iter () { let segment = module . elements . get (segment) ; let offset = match & segment . kind { walrus :: ElementKind :: Active { offset : ConstExpr :: Value (Value :: I32 (n)) , .. } => * n as u32 , _ => continue , } ; let idx = (idx - offset) as usize ; let slot = match & segment . items { ElementItems :: Functions (items) => items . get (idx) . map (Some) , ElementItems :: Expressions (_ , items) => items . get (idx) . map (| item | { if let ConstExpr :: RefFunc (target) = item { Some (target) } else { None } }) , } ; match slot { Some (slot) => { return slot . copied () . context ("function table entry wasn't filled") ; } None => continue , } } bail ! ("failed to find `{idx}` in function table") ; }
};
}
