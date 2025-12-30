// Generated macro for generate_c_test_loop (function)
macro_rules! Depcrate_common_gen_cgenerate_c_test_loop {
() => {
// Module: crate::common::gen_c
// Provides: {"generate_c_test_loop"}
// Dependencies: {}
pub fn generate_c_test_loop < T : IntrinsicTypeDefinition + Sized > (w : & mut impl std :: io :: Write , intrinsic : & Intrinsic < T > , indentation : Indentation , additional : & str , passes : u32 ,) -> std :: io :: Result < () > { let body_indentation = indentation . nested () ; writeln ! (w , "{indentation}for (int i=0; i<{passes}; i++) {{\n\
            {loaded_args}\
            {body_indentation}auto __return_value = {intrinsic_call}({args});\n\
            {print_result}\n\
        {indentation}}}" , loaded_args = intrinsic . arguments . load_values_c (body_indentation) , intrinsic_call = intrinsic . name , args = intrinsic . arguments . as_call_param_c () , print_result = intrinsic . results . print_result_c (body_indentation , additional)) }
};
}
