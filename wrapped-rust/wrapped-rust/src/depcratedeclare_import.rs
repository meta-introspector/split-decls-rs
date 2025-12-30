// Generated macro for declare_import (function)
macro_rules! Depcratedeclare_import {
() => {
// Module: crate
// Provides: {"declare_import"}
// Dependencies: {}
fn declare_import (wasm_import_module : & str , wasm_import_name : & str , rust_name : & str , params : & [WasmType] , results : & [WasmType] ,) -> String { let mut sig = "(" . to_owned () ; for param in params . iter () { sig . push_str ("_: ") ; sig . push_str (wasm_type (* param)) ; sig . push_str (", ") ; } sig . push (')') ; assert ! (results . len () < 2) ; for result in results . iter () { sig . push_str (" -> ") ; sig . push_str (wasm_type (* result)) ; } format ! ("
            #[cfg(target_arch = \"wasm32\")]
            #[link(wasm_import_module = \"{wasm_import_module}\")]
            unsafe extern \"C\" {{
                #[link_name = \"{wasm_import_name}\"]
                fn {rust_name}{sig};
            }}

            #[cfg(not(target_arch = \"wasm32\"))]
            unsafe extern \"C\" fn {rust_name}{sig} {{ unreachable!() }}
        ") }
};
}
