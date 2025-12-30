// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'a , 'b > FunctionBindgen < 'a , 'b > { fn new (gen : & 'b mut InterfaceGenerator < 'a > , func_name : & 'b str , func_interface : & 'b str , params : Box < [String] > ,) -> FunctionBindgen < 'a , 'b > { let mut locals = Ns :: default () ; params . iter () . for_each (| str | { locals . tmp (str) ; }) ; Self { gen , func_name , func_interface , params , src : String :: new () , locals , block_storage : Vec :: new () , blocks : Vec :: new () , payloads : Vec :: new () , cleanup : Vec :: new () , needs_cleanup_list : false , deferred_task_return : DeferredTaskReturn :: None , } } fn lower_variant (& mut self , cases : & [(& str , Option < Type >)] , lowered_types : & [WasmType] , op : & str , results : & mut Vec < String > , is_result : bool ,) { let blocks = self . blocks . drain (self . blocks . len () - cases . len () ..) . collect :: < Vec < _ > > () ; let payloads = self . payloads . drain (self . payloads . len () - cases . len () ..) . collect :: < Vec < _ > > () ; let lowered = lowered_types . iter () . map (| _ | self . locals . tmp ("lowered")) . collect :: < Vec < _ > > () ; results . extend (lowered . iter () . cloned ()) ; let declarations = lowered . join (",") ; let cases = cases . iter () . zip (blocks) . zip (payloads) . map (| (((name , ty) , Block { body , results , .. }) , payload) | { let name = name . to_upper_camel_case () ; let assignments = results . iter () . map (| result | result . to_string ()) . collect :: < Vec < _ > > () . join (", ") ; let payload = if self . gen . non_empty_type (ty . as_ref ()) . is_some () { payload } else if is_result { format ! ("_{payload}") } else { String :: new () } ; if payload . is_empty () { format ! ("{name} => {{
                          {body}
                          ({assignments})
                        }}") } else { format ! ("{name}({payload}) => {{
                          {body}
                          ({assignments})
                        }}" ,) } }) . collect :: < Vec < _ > > () . join ("\n") ; if declarations . is_empty () { uwrite ! (self . src , r#"
                match {op} {{
                    {cases}
                }}
                "#) ; } else { uwrite ! (self . src , r#"
                let ({declarations}) = match {op} {{
                    {cases}
                }}
                "#) ; } } fn lift_variant (& mut self , ty : & Type , cases : & [(& str , Option < Type >)] , op : & str , results : & mut Vec < String > , is_result : bool ,) { let blocks = self . blocks . drain (self . blocks . len () - cases . len () ..) . collect :: < Vec < _ > > () ; let ty = self . gen . type_name (ty , false) ; let lifted = self . locals . tmp ("lifted") ; let cases = cases . iter () . zip (blocks) . enumerate () . map (| (i , ((case_name , case_ty) , Block { body , results , .. })) | { let payload = if self . gen . non_empty_type (case_ty . as_ref ()) . is_some () { results . into_iter () . next () . unwrap () } else { String :: new () } ; let constructor = format ! ("{ty}::{}" , case_name . to_upper_camel_case ()) ; if payload . is_empty () && ! is_result { format ! ("{i} => {{
                             {body}
                             {constructor}
                         }}") } else { format ! ("{i} => {{
                             {body}
                             {constructor}({})
                         }}" , if payload . is_empty () { "()" . into () } else { payload }) } }) . collect :: < Vec < _ > > () . join ("\n") ; uwrite ! (self . src , r#"
            let {lifted} = match ({op}) {{
                {cases}
                _ => panic()
            }}
            "#) ; results . push (lifted) ; } }
};
}
