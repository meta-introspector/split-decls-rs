macro_rules! wasm_functype {
    () => {
        # [doc = " The webassembly type signature for the given function."] # [doc = ""] # [doc = " Used by the `.functype` directive on wasm targets."] fn wasm_functype < 'tcx > (tcx : TyCtxt < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > >) -> String { let mut signature = String :: with_capacity (64) ; let ptr_type = match tcx . data_layout . pointer_size () . bits () { 32 => "i32" , 64 => "i64" , other => bug ! ("wasm pointer size cannot be {other} bits") , } ; let hidden_return = matches ! (fn_abi . ret . mode , PassMode :: Indirect { .. }) ; signature . push ('(') ; if hidden_return { signature . push_str (ptr_type) ; if ! fn_abi . args . is_empty () { signature . push_str (", ") ; } } let mut it = fn_abi . args . iter () . peekable () ; while let Some (arg_abi) = it . next () { wasm_type (& mut signature , arg_abi , ptr_type) ; if it . peek () . is_some () { signature . push_str (", ") ; } } signature . push_str (") -> (") ; if ! hidden_return { wasm_type (& mut signature , & fn_abi . ret , ptr_type) ; } signature . push (')') ; signature }
    };
}

wasm_functype!()