macro_rules! deps {
    () => {
        DemangleAsInner!();
        DemangleWrite!();
        DemangleState!();
        SubstitutionTable!();
    };
}

macro_rules! DemangleContext {
    () => {
        deps!();
        # [doc = " Common state that is required when demangling a mangled symbol's parsed AST."] # [doc (hidden)] # [derive (Debug)] pub struct DemangleContext < 'a , W > where W : 'a + DemangleWrite , { subs : & 'a SubstitutionTable , max_recursion : u32 , inner : Vec < & 'a dyn DemangleAsInner < 'a , W > > , input : & 'a [u8] , source_name : Option < & 'a str > , out : & 'a mut W , bytes_written : usize , last_char_written : Option < char > , is_lambda_arg : bool , is_template_prefix : bool , is_template_prefix_in_nested_name : bool , is_template_argument_pack : bool , is_explicit_obj_param : bool , show_params : bool , show_return_type : bool , show_expression_literal_types : bool , state : Cell < DemangleState > , }
    };
}

DemangleContext!()