mkuse!{use std :: sync :: Arc ;}
mkuse!{use rustc_expand :: base :: { MacroExpanderFn , ResolverExpand , SyntaxExtensionKind } ;}
mkuse!{use rustc_expand :: proc_macro :: BangProcMacro ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use crate :: deriving :: * ;}
mkmod!{alloc_error_handler, { 
                getname!(alloc_error_handler);
                getsrc!(alloc_error_handler);
                getpath!(alloc_error_handler);
                get_deps!(alloc_error_handler);
                get_crates!(alloc_error_handler);
                mkinclude!(alloc_error_handler);
                 
            }}
mkmod!{assert, { 
                getname!(assert);
                getsrc!(assert);
                getpath!(assert);
                get_deps!(assert);
                get_crates!(assert);
                mkinclude!(assert);
                 
            }}
mkmod!{autodiff, { 
                getname!(autodiff);
                getsrc!(autodiff);
                getpath!(autodiff);
                get_deps!(autodiff);
                get_crates!(autodiff);
                mkinclude!(autodiff);
                 
            }}
mkmod!{cfg, { 
                getname!(cfg);
                getsrc!(cfg);
                getpath!(cfg);
                get_deps!(cfg);
                get_crates!(cfg);
                mkinclude!(cfg);
                 
            }}
mkmod!{cfg_accessible, { 
                getname!(cfg_accessible);
                getsrc!(cfg_accessible);
                getpath!(cfg_accessible);
                get_deps!(cfg_accessible);
                get_crates!(cfg_accessible);
                mkinclude!(cfg_accessible);
                 
            }}
mkmod!{cfg_eval, { 
                getname!(cfg_eval);
                getsrc!(cfg_eval);
                getpath!(cfg_eval);
                get_deps!(cfg_eval);
                get_crates!(cfg_eval);
                mkinclude!(cfg_eval);
                 
            }}
mkmod!{cfg_select, { 
                getname!(cfg_select);
                getsrc!(cfg_select);
                getpath!(cfg_select);
                get_deps!(cfg_select);
                get_crates!(cfg_select);
                mkinclude!(cfg_select);
                 
            }}
mkmod!{compile_error, { 
                getname!(compile_error);
                getsrc!(compile_error);
                getpath!(compile_error);
                get_deps!(compile_error);
                get_crates!(compile_error);
                mkinclude!(compile_error);
                 
            }}
mkmod!{concat, { 
                getname!(concat);
                getsrc!(concat);
                getpath!(concat);
                get_deps!(concat);
                get_crates!(concat);
                mkinclude!(concat);
                 
            }}
mkmod!{concat_bytes, { 
                getname!(concat_bytes);
                getsrc!(concat_bytes);
                getpath!(concat_bytes);
                get_deps!(concat_bytes);
                get_crates!(concat_bytes);
                mkinclude!(concat_bytes);
                 
            }}
mkmod!{define_opaque, { 
                getname!(define_opaque);
                getsrc!(define_opaque);
                getpath!(define_opaque);
                get_deps!(define_opaque);
                get_crates!(define_opaque);
                mkinclude!(define_opaque);
                 
            }}
mkmod!{derive, { 
                getname!(derive);
                getsrc!(derive);
                getpath!(derive);
                get_deps!(derive);
                get_crates!(derive);
                mkinclude!(derive);
                 
            }}
mkmod!{deriving, { 
                getname!(deriving);
                getsrc!(deriving);
                getpath!(deriving);
                get_deps!(deriving);
                get_crates!(deriving);
                mkinclude!(deriving);
                 
            }}
mkmod!{edition_panic, { 
                getname!(edition_panic);
                getsrc!(edition_panic);
                getpath!(edition_panic);
                get_deps!(edition_panic);
                get_crates!(edition_panic);
                mkinclude!(edition_panic);
                 
            }}
mkmod!{env, { 
                getname!(env);
                getsrc!(env);
                getpath!(env);
                get_deps!(env);
                get_crates!(env);
                mkinclude!(env);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{format, { 
                getname!(format);
                getsrc!(format);
                getpath!(format);
                get_deps!(format);
                get_crates!(format);
                mkinclude!(format);
                 
            }}
mkmod!{format_foreign, { 
                getname!(format_foreign);
                getsrc!(format_foreign);
                getpath!(format_foreign);
                get_deps!(format_foreign);
                get_crates!(format_foreign);
                mkinclude!(format_foreign);
                 
            }}
mkmod!{global_allocator, { 
                getname!(global_allocator);
                getsrc!(global_allocator);
                getpath!(global_allocator);
                get_deps!(global_allocator);
                get_crates!(global_allocator);
                mkinclude!(global_allocator);
                 
            }}
mkmod!{iter, { 
                getname!(iter);
                getsrc!(iter);
                getpath!(iter);
                get_deps!(iter);
                get_crates!(iter);
                mkinclude!(iter);
                 
            }}
mkmod!{log_syntax, { 
                getname!(log_syntax);
                getsrc!(log_syntax);
                getpath!(log_syntax);
                get_deps!(log_syntax);
                get_crates!(log_syntax);
                mkinclude!(log_syntax);
                 
            }}
mkmod!{pattern_type, { 
                getname!(pattern_type);
                getsrc!(pattern_type);
                getpath!(pattern_type);
                get_deps!(pattern_type);
                get_crates!(pattern_type);
                mkinclude!(pattern_type);
                 
            }}
mkmod!{source_util, { 
                getname!(source_util);
                getsrc!(source_util);
                getpath!(source_util);
                get_deps!(source_util);
                get_crates!(source_util);
                mkinclude!(source_util);
                 
            }}
mkmod!{test, { 
                getname!(test);
                getsrc!(test);
                getpath!(test);
                get_deps!(test);
                get_crates!(test);
                mkinclude!(test);
                 
            }}
mkmod!{trace_macros, { 
                getname!(trace_macros);
                getsrc!(trace_macros);
                getpath!(trace_macros);
                get_deps!(trace_macros);
                get_crates!(trace_macros);
                mkinclude!(trace_macros);
                 
            }}
mkmod!{asm, { 
                getname!(asm);
                getsrc!(asm);
                getpath!(asm);
                get_deps!(asm);
                get_crates!(asm);
                mkinclude!(asm);
                 
            }}
mkmod!{cmdline_attrs, { 
                getname!(cmdline_attrs);
                getsrc!(cmdline_attrs);
                getpath!(cmdline_attrs);
                get_deps!(cmdline_attrs);
                get_crates!(cmdline_attrs);
                mkinclude!(cmdline_attrs);
                 
            }}
mkmod!{contracts, { 
                getname!(contracts);
                getsrc!(contracts);
                getpath!(contracts);
                get_deps!(contracts);
                get_crates!(contracts);
                mkinclude!(contracts);
                 
            }}
mkmod!{proc_macro_harness, { 
                getname!(proc_macro_harness);
                getsrc!(proc_macro_harness);
                getpath!(proc_macro_harness);
                get_deps!(proc_macro_harness);
                get_crates!(proc_macro_harness);
                mkinclude!(proc_macro_harness);
                 
            }}
mkmod!{standard_library_imports, { 
                getname!(standard_library_imports);
                getsrc!(standard_library_imports);
                getpath!(standard_library_imports);
                get_deps!(standard_library_imports);
                get_crates!(standard_library_imports);
                mkinclude!(standard_library_imports);
                 
            }}
mkmod!{test_harness, { 
                getname!(test_harness);
                getsrc!(test_harness);
                getpath!(test_harness);
                get_deps!(test_harness);
                get_crates!(test_harness);
                mkinclude!(test_harness);
                 
            }}
mkmod!{util, { 
                getname!(util);
                getsrc!(util);
                getpath!(util);
                get_deps!(util);
                get_crates!(util);
                mkinclude!(util);
                 
            }}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}

macro_rules! register_builtin_macros_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function register_builtin_macros in module {}", module_path!());
    };
}

mkfn!{
    register_builtin_macros_introspect!();
    pub fn register_builtin_macros (resolver : & mut dyn ResolverExpand) { let mut register = | name , kind | resolver . register_builtin_macro (name , kind) ; macro register_bang ($ ($ name : ident : $ f : expr ,) *) { $ (register (sym ::$ name , SyntaxExtensionKind :: LegacyBang (Arc :: new ($ f as MacroExpanderFn))) ;) * } macro register_attr ($ ($ name : ident : $ f : expr ,) *) { $ (register (sym ::$ name , SyntaxExtensionKind :: LegacyAttr (Arc :: new ($ f))) ;) * } macro register_derive ($ ($ name : ident : $ f : expr ,) *) { $ (register (sym ::$ name , SyntaxExtensionKind :: LegacyDerive (Arc :: new (BuiltinDerive ($ f)))) ;) * } register_bang ! { asm : asm :: expand_asm , assert : assert :: expand_assert , cfg : cfg :: expand_cfg , cfg_select : cfg_select :: expand_cfg_select , column : source_util :: expand_column , compile_error : compile_error :: expand_compile_error , concat : concat :: expand_concat , concat_bytes : concat_bytes :: expand_concat_bytes , const_format_args : format :: expand_format_args , core_panic : edition_panic :: expand_panic , env : env :: expand_env , file : source_util :: expand_file , format_args : format :: expand_format_args , format_args_nl : format :: expand_format_args_nl , global_asm : asm :: expand_global_asm , include : source_util :: expand_include , include_bytes : source_util :: expand_include_bytes , include_str : source_util :: expand_include_str , iter : iter :: expand , line : source_util :: expand_line , log_syntax : log_syntax :: expand_log_syntax , module_path : source_util :: expand_mod , naked_asm : asm :: expand_naked_asm , option_env : env :: expand_option_env , pattern_type : pattern_type :: expand , std_panic : edition_panic :: expand_panic , stringify : source_util :: expand_stringify , trace_macros : trace_macros :: expand_trace_macros , unreachable : edition_panic :: expand_unreachable , } register_attr ! { alloc_error_handler : alloc_error_handler :: expand , autodiff_forward : autodiff :: expand_forward , autodiff_reverse : autodiff :: expand_reverse , bench : test :: expand_bench , cfg_accessible : cfg_accessible :: Expander , cfg_eval : cfg_eval :: expand , define_opaque : define_opaque :: expand , derive : derive :: Expander { is_const : false } , derive_const : derive :: Expander { is_const : true } , global_allocator : global_allocator :: expand , test : test :: expand_test , test_case : test :: expand_test_case , } register_derive ! { Clone : clone :: expand_deriving_clone , Copy : bounds :: expand_deriving_copy , ConstParamTy : bounds :: expand_deriving_const_param_ty , UnsizedConstParamTy : bounds :: expand_deriving_unsized_const_param_ty , Debug : debug :: expand_deriving_debug , Default : default :: expand_deriving_default , Eq : eq :: expand_deriving_eq , Hash : hash :: expand_deriving_hash , Ord : ord :: expand_deriving_ord , PartialEq : partial_eq :: expand_deriving_partial_eq , PartialOrd : partial_ord :: expand_deriving_partial_ord , CoercePointee : coerce_pointee :: expand_deriving_coerce_pointee , From : from :: expand_deriving_from , } let client = rustc_proc_macro :: bridge :: client :: Client :: expand1 (rustc_proc_macro :: quote) ; register (sym :: quote , SyntaxExtensionKind :: Bang (Arc :: new (BangProcMacro { client }))) ; let requires = SyntaxExtensionKind :: Attr (Arc :: new (contracts :: ExpandRequires)) ; register (sym :: contracts_requires , requires) ; let ensures = SyntaxExtensionKind :: Attr (Arc :: new (contracts :: ExpandEnsures)) ; register (sym :: contracts_ensures , ensures) ; }
}