macro_rules! deps {
    () => {
        Cycle!();
        TrackedQuery!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl ToTokens for TrackedQuery { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let sig = & self . signature ; let trait_name = & self . trait_name ; let ret = & sig . output ; let invoke = match & self . invoke { Some (path) => path . to_token_stream () , None => sig . ident . to_token_stream () , } ; let fn_ident = & sig . ident ; let shim : Ident = format_ident ! ("{}_shim" , fn_ident) ; let options = self . cycle . as_ref () . map (| Cycle { cycle_fn , cycle_initial , cycle_result } | { let cycle_fn = cycle_fn . as_ref () . map (| (ident , path) | quote ! (# ident =# path)) ; let cycle_initial = cycle_initial . as_ref () . map (| (ident , path) | quote ! (# ident =# path)) ; let cycle_result = cycle_result . as_ref () . map (| (ident , path) | quote ! (# ident =# path)) ; let options = cycle_fn . into_iter () . chain (cycle_initial) . chain (cycle_result) ; quote ! (# (# options) ,*) }) . into_iter () . chain (self . lru . map (| lru | quote ! (lru = # lru))) . chain (Some (quote ! (unsafe (non_update_return_type)))) ; let annotation = quote ! (# [salsa_macros :: tracked (# (# options) ,*)]) ; let pat_and_tys = & self . pat_and_tys ; let params = self . pat_and_tys . iter () . map (| pat_type | pat_type . pat . clone ()) . collect :: < Vec < Box < syn :: Pat > > > () ; let invoke_block = match & self . default { Some (default) => quote ! { # default } , None => { let invoke_params : proc_macro2 :: TokenStream = quote ! { db , # (# params) ,* } ; quote_spanned ! { invoke . span () => { # invoke (# invoke_params) } } } } ; let method = match & self . generated_struct { Some (generated_struct) => { let input_struct_name = & generated_struct . input_struct_name ; let create_data_ident = & generated_struct . create_data_ident ; quote ! { # sig { # annotation fn # shim <'db > (db : &'db dyn # trait_name , _input : # input_struct_name , # (# pat_and_tys) ,*) # ret # invoke_block # shim (self , # create_data_ident (self) , # (# params) ,*) } } } None => { quote ! { # sig { # annotation fn # shim <'db > (db : &'db dyn # trait_name , # (# pat_and_tys) ,*) # ret # invoke_block # shim (self , # (# params) ,*) } } } } ; method . to_tokens (tokens) ; } }
    };
}

impl_2!()