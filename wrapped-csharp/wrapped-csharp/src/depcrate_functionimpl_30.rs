// Generated macro for impl_30 (impl)
macro_rules! Depcrate_functionimpl_30 {
() => {
// Module: crate::function
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a , 'b > FunctionBindgen < 'a , 'b > { pub (crate) fn new (interface_gen : & 'b mut InterfaceGenerator < 'a > , func_name : & 'b str , kind : & 'b FunctionKind , params : Box < [String] > , results : Vec < TypeId > , parameter_type : ParameterType , result_type : Option < Type > ,) -> FunctionBindgen < 'a , 'b > { let mut locals = Ns :: default () ; for param in & params [..] { locals . tmp (param) ; } Self { interface_gen , func_name , kind , params , results , src : String :: new () , locals , block_storage : Vec :: new () , blocks : Vec :: new () , payloads : Vec :: new () , needs_cleanup : false , import_return_pointer_area_size : 0 , import_return_pointer_area_align : 0 , resource_drops : Vec :: new () , is_block : false , fixed_statments : Vec :: new () , parameter_type : parameter_type , result_type : result_type , } } fn lower_variant (& mut self , cases : & [(& str , Option < Type >)] , lowered_types : & [WasmType] , op : & str , results : & mut Vec < String > ,) { let blocks = self . blocks . drain (self . blocks . len () - cases . len () ..) . collect :: < Vec < _ > > () ; let payloads = self . payloads . drain (self . payloads . len () - cases . len () ..) . collect :: < Vec < _ > > () ; let lowered = lowered_types . iter () . map (| _ | self . locals . tmp ("lowered")) . collect :: < Vec < _ > > () ; results . extend (lowered . iter () . cloned ()) ; let declarations = lowered . iter () . zip (lowered_types) . map (| (lowered , ty) | format ! ("{} {lowered};" , crate :: world_generator :: wasm_type (* ty))) . collect :: < Vec < _ > > () . join ("\n") ; let cases = cases . iter () . zip (blocks) . zip (payloads) . enumerate () . map (| (i , (((name , ty) , Block { body , results , .. }) , payload)) | { let payload = if let Some (ty) = self . interface_gen . non_empty_type (ty . as_ref ()) { let ty = self . interface_gen . type_name_with_qualifier (ty , true) ; let name = name . to_upper_camel_case () ; format ! ("{ty} {payload} = {op}.As{name};") } else { String :: new () } ; let assignments = lowered . iter () . zip (& results) . map (| (lowered , result) | format ! ("{lowered} = {result};\n")) . collect :: < Vec < _ > > () . concat () ; format ! ("case {i}: {{
                         {payload}
                         {body}
                         {assignments}
                         break;
                     }}") } ,) . collect :: < Vec < _ > > () . join ("\n") ; uwrite ! (self . src , r#"
            {declarations}

            switch ({op}.Tag) {{
                {cases}

                default: throw new global::System.ArgumentException("invalid discriminant: " + {op});
            }}
            "#) ; } fn lift_variant (& mut self , ty : & Type , cases : & [(& str , Option < Type >)] , op : & str , results : & mut Vec < String > ,) { let blocks = self . blocks . drain (self . blocks . len () - cases . len () ..) . collect :: < Vec < _ > > () ; let ty = self . interface_gen . type_name_with_qualifier (ty , true) ; let generics_position = ty . find ('<') ; let lifted = self . locals . tmp ("lifted") ; let cases = cases . iter () . zip (blocks) . enumerate () . map (| (i , ((case_name , case_ty) , Block { body , results , .. })) | { let payload = if self . interface_gen . non_empty_type (case_ty . as_ref ()) . is_some () { results . into_iter () . next () . unwrap () } else if generics_position . is_some () { if let Some (ty) = case_ty . as_ref () { format ! ("{}.INSTANCE" , self . interface_gen . type_name_with_qualifier (ty , true)) } else { format ! ("new global::{}None()" , self . interface_gen . csharp_gen . qualifier ()) } } else { String :: new () } ; let method = case_name . to_csharp_ident_upper () ; let call = if let Some (position) = generics_position { let (ty , generics) = ty . split_at (position) ; format ! ("{ty}{generics}.{method}") } else { format ! ("{ty}.{method}") } ; format ! ("case {i}: {{
                         {body}
                         {lifted} = {call}({payload});
                         break;
                     }}") }) . collect :: < Vec < _ > > () . join ("\n") ; uwrite ! (self . src , r#"
            {ty} {lifted};

            switch ({op}) {{
                {cases}

                default: throw new global::System.ArgumentException("invalid discriminant:" + {op});
            }}
            "#) ; results . push (lifted) ; } fn handle_result_import (& mut self , operands : & mut Vec < String >) { if self . interface_gen . csharp_gen . opts . with_wit_results { uwriteln ! (self . src , "return {};" , operands [0]) ; return ; } let mut payload_is_void = false ; let mut previous = operands [0] . clone () ; let mut vars : Vec < (String , Option < String >) > = Vec :: with_capacity (self . results . len ()) ; if let Direction :: Import = self . interface_gen . direction { for ty in & self . results { let tmp = self . locals . tmp ("tmp") ; uwrite ! (self . src , "\
                    if ({previous}.IsOk)
                    {{
                        var {tmp} = {previous}.AsOk;
                    ") ; let TypeDefKind :: Result (result) = & self . interface_gen . resolve . types [* ty] . kind else { unreachable ! () ; } ; let exception_name = result . err . map (| ty | self . interface_gen . type_name_with_qualifier (& ty , true)) ; vars . push ((previous . clone () , exception_name)) ; payload_is_void = result . ok . is_none () ; previous = tmp ; } } uwriteln ! (self . src , "return {};" , if payload_is_void { "" } else { & previous }) ; for (level , var) in vars . iter () . enumerate () . rev () { self . interface_gen . csharp_gen . needs_wit_exception = true ; let (var_name , exception_name) = var ; let exception_name = match exception_name { Some (type_name) => & format ! ("WitException<{}>" , type_name) , None => "WitException" , } ; uwrite ! (self . src , "\
                }}
                else
                {{
                    throw new {exception_name}({var_name}.AsErr!, {level});
                }}
                ") ; } } fn handle_result_call (& mut self , func : & & wit_parser :: Function , target : String , func_name : String , oper : String ,) -> String { let ret = self . locals . tmp ("ret") ; if self . interface_gen . csharp_gen . opts . with_wit_results { uwriteln ! (self . src , "var {ret} = {target}.{func_name}({oper});") ; return ret ; } let ty = self . interface_gen . type_name_with_qualifier (& func . result . unwrap () , true) ; let is_async = InterfaceGenerator :: is_async (& func . kind) ; if is_async { uwriteln ! (self . src , "Task<{ty}> {ret};") ; } else { uwriteln ! (self . src , "{ty} {ret};") ; } let mut cases = Vec :: with_capacity (self . results . len ()) ; let mut oks = Vec :: with_capacity (self . results . len ()) ; let mut payload_is_void = false ; for (index , ty) in self . results . iter () . enumerate () { let TypeDefKind :: Result (result) = & self . interface_gen . resolve . types [* ty] . kind else { unreachable ! () ; } ; let err_ty = if let Some (ty) = result . err { self . interface_gen . type_name_with_qualifier (& ty , true) } else { "None" . to_owned () } ; let ty = self . interface_gen . type_name_with_qualifier (& Type :: Id (* ty) , true) ; let head = oks . concat () ; let tail = oks . iter () . map (| _ | ")") . collect :: < Vec < _ > > () . concat () ; cases . push (format ! ("\
                case {index}:
                {{
                    ret = {head}{ty}.Err(({err_ty}) e.Value){tail};
                    break;
                }}
                ")) ; oks . push (format ! ("{ty}.Ok(")) ; payload_is_void = result . ok . is_none () ; } if ! self . results . is_empty () { self . src . push_str ("
                try
                {\n
                " ,) ; } let head = oks . concat () ; let tail = oks . iter () . map (| _ | ")") . collect :: < Vec < _ > > () . concat () ; let val = if payload_is_void { uwriteln ! (self . src , "{target}.{func_name}({oper});") ; "new None()" . to_owned () } else { format ! ("{target}.{func_name}({oper})") } ; uwriteln ! (self . src , "{ret} = {head}{val}{tail};") ; if ! self . results . is_empty () { self . interface_gen . csharp_gen . needs_wit_exception = true ; let cases = cases . join ("\n") ; uwriteln ! (self . src , r#"}}
                    catch (WitException e)
                    {{
                        switch (e.NestingLevel)
                        {{
                            {cases}

                            default: throw new global::System.ArgumentException($"invalid nesting level: {{e.NestingLevel}}");
                        }}
                    }}
                "#) ; } ret } fn emit_allocation_for_type (& mut self , results : & [WasmType]) -> String { let address = self . locals . tmp ("address") ; let buffer_size = self . get_size_for_type (results) ; let align = self . get_align_for_type (results) ; uwriteln ! (self . src , "void* {address} = global::System.Runtime.InteropServices.NativeMemory.AlignedAlloc({buffer_size}, {align});") ; address } fn get_size_for_type (& self , results : & [WasmType]) -> usize { match results { [WasmType :: I32] => 4 , [WasmType :: I64] => 8 , [WasmType :: F32] => 4 , [WasmType :: F64] => 8 , [WasmType :: Pointer , WasmType :: Length] => 4 , [WasmType :: PointerOrI64 , WasmType :: Length] => 8 , _ => { todo ! ("other types not yet supported") ; } } } fn get_align_for_type (& self , results : & [WasmType]) -> usize { match results { [WasmType :: I32] => 4 , [WasmType :: I64] => 8 , [WasmType :: F32] => 4 , [WasmType :: F64] => 8 , [WasmType :: Pointer , WasmType :: Length] => 4 , [WasmType :: PointerOrI64 , WasmType :: Length] => 8 , _ => { todo ! ("other types not yet supported") ; } } } }
};
}
