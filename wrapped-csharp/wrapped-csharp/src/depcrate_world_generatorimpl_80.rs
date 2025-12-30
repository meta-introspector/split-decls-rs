// Generated macro for impl_80 (impl)
macro_rules! Depcrate_world_generatorimpl_80 {
() => {
// Module: crate::world_generator
// Provides: {"impl_80"}
// Dependencies: {}
impl WorldGenerator for CSharp { fn preprocess (& mut self , resolve : & Resolve , world : WorldId) { let name = & resolve . worlds [world] . name ; self . name = name . to_string () ; self . sizes . fill (resolve) ; } fn import_interface (& mut self , resolve : & Resolve , key : & WorldKey , id : InterfaceId , _files : & mut Files ,) -> anyhow :: Result < () > { let name = interface_name (self , resolve , key , Direction :: Import) ; self . interface_names . insert (id , name . clone ()) ; let mut gen = self . interface (resolve , & name , Direction :: Import) ; let mut old_resources = mem :: take (& mut gen . csharp_gen . all_resources) ; gen . types (id) ; let new_resources = mem :: take (& mut gen . csharp_gen . all_resources) ; old_resources . extend (new_resources . clone ()) ; gen . csharp_gen . all_resources = old_resources ; for (resource , funcs) in by_resource (resolve . interfaces [id] . functions . iter () . map (| (k , v) | (k . as_str () , v)) , new_resources . keys () . copied () ,) { if let Some (resource) = resource { gen . start_resource (resource , Some (key)) ; } let import_module_name = & resolve . name_world_key (key) ; for func in funcs { gen . import (import_module_name , func) ; } if resource . is_some () { gen . end_resource () ; } } gen . define_interface_types (id) ; gen . add_interface_fragment (false) ; Ok (()) } fn import_funcs (& mut self , resolve : & Resolve , world : WorldId , funcs : & [(& str , & Function)] , _files : & mut Files ,) { self . import_funcs_called = true ; let name = & format ! ("{}-world" , resolve . worlds [world] . name) . to_upper_camel_case () ; let name = & format ! ("{name}.I{name}") ; let mut gen = self . interface (resolve , name , Direction :: Import) ; for (resource , funcs) in by_resource (funcs . iter () . copied () , gen . csharp_gen . world_resources . keys () . copied () ,) { if let Some (resource) = resource { gen . start_resource (resource , None) ; } for func in funcs { gen . import ("$root" , func) ; } if resource . is_some () { gen . end_resource () ; } } gen . add_world_fragment () ; } fn export_interface (& mut self , resolve : & Resolve , key : & WorldKey , id : InterfaceId , _files : & mut Files ,) -> anyhow :: Result < () > { let name = interface_name (self , resolve , key , Direction :: Export) ; self . interface_names . insert (id , name . clone ()) ; let mut gen = self . interface (resolve , & name , Direction :: Export) ; let mut old_resources = mem :: take (& mut gen . csharp_gen . all_resources) ; gen . types (id) ; let new_resources = mem :: take (& mut gen . csharp_gen . all_resources) ; old_resources . extend (new_resources . clone ()) ; gen . csharp_gen . all_resources = old_resources ; for (resource , funcs) in by_resource (resolve . interfaces [id] . functions . iter () . map (| (k , v) | (k . as_str () , v)) , new_resources . keys () . copied () ,) { if let Some (resource) = resource { gen . start_resource (resource , Some (key)) ; } for func in funcs { gen . export (func , Some (key)) ; } if resource . is_some () { gen . end_resource () ; } } gen . define_interface_types (id) ; gen . add_interface_fragment (true) ; Ok (()) } fn export_funcs (& mut self , resolve : & Resolve , world : WorldId , funcs : & [(& str , & Function)] , _files : & mut Files ,) -> anyhow :: Result < () > { let name = & format ! ("{}-world" , resolve . worlds [world] . name) . to_upper_camel_case () ; let name = & format ! ("{name}.I{name}") ; let mut gen = self . interface (resolve , name , Direction :: Export) ; for (resource , funcs) in by_resource (funcs . iter () . copied () , iter :: empty ()) { if let Some (resource) = resource { gen . start_resource (resource , None) ; } for func in funcs { gen . export (func , None) ; } if resource . is_some () { gen . end_resource () ; } } gen . add_world_fragment () ; Ok (()) } fn import_types (& mut self , resolve : & Resolve , world : WorldId , types : & [(& str , TypeId)] , _files : & mut Files ,) { let name = & format ! ("{}-world" , resolve . worlds [world] . name) . to_upper_camel_case () ; let name = & format ! ("{name}.I{name}") ; let mut gen = self . interface (resolve , name , Direction :: Import) ; let mut old_resources = mem :: take (& mut gen . csharp_gen . all_resources) ; for (ty_name , ty) in types { gen . define_type (ty_name , * ty) ; } let new_resources = mem :: take (& mut gen . csharp_gen . all_resources) ; old_resources . extend (new_resources . clone ()) ; gen . csharp_gen . all_resources = old_resources ; gen . csharp_gen . world_resources = new_resources ; gen . add_world_fragment () ; } fn finish (& mut self , resolve : & Resolve , id : WorldId , files : & mut Files) -> anyhow :: Result < () > { if ! self . import_funcs_called { self . import_funcs (resolve , id , & [] , files) ; } let world = & resolve . worlds [id] ; let world_namespace = self . qualifier () ; let world_namespace = world_namespace . strip_suffix (".") . unwrap () ; let namespace = format ! ("{world_namespace}") ; let name = world . name . to_upper_camel_case () ; let version = env ! ("CARGO_PKG_VERSION") ; let header = format ! ("\
            // Generated by `wit-bindgen` {version}. DO NOT EDIT!
            // <auto-generated />
            #nullable enable
            ") ; let mut src = String :: new () ; src . push_str (& header) ; let access = self . access_modifier () ; uwrite ! (src , "
             namespace {world_namespace} {{

             {access} interface I{name}World {{
            ") ; src . push_str (& self . world_fragments . iter () . map (| f | f . csharp_src . deref ()) . collect :: < Vec < _ > > () . join ("\n") ,) ; let mut producers = wasm_metadata :: Producers :: empty () ; producers . add ("processed-by" , env ! ("CARGO_PKG_NAME") , env ! ("CARGO_PKG_VERSION") ,) ; src . push_str ("}\n") ; if self . needs_result { uwrite ! (src , r#"

                {access} readonly struct None {{}}

                [global::System.Runtime.InteropServices.StructLayoutAttribute(global::System.Runtime.InteropServices.LayoutKind.Sequential)]
                {access} readonly struct Result<TOk, TErr>
                {{
                    {access} readonly byte Tag;
                    private readonly object value;

                    private Result(byte tag, object value)
                    {{
                        Tag = tag;
                        this.value = value;
                    }}

                    {access} static Result<TOk, TErr> Ok(TOk ok)
                    {{
                        return new Result<TOk, TErr>(Tags.Ok, ok!);
                    }}

                    {access} static Result<TOk, TErr> Err(TErr err)
                    {{
                        return new Result<TOk, TErr>(Tags.Err, err!);
                    }}

                    {access} bool IsOk => Tag == Tags.Ok;
                    {access} bool IsErr => Tag == Tags.Err;

                    {access} TOk AsOk
                    {{
                        get
                        {{
                            if (Tag == Tags.Ok)
                            {{
                                return (TOk)value;
                            }}

                            throw new global::System.ArgumentException("expected k, got " + Tag);
                        }}
                    }}

                    {access} TErr AsErr
                    {{
                        get
                        {{
                            if (Tag == Tags.Err)
                            {{
                                return (TErr)value;
                            }}

                            throw new global::System.ArgumentException("expected Err, got " + Tag);
                        }}
                    }}

                    {access} class Tags
                    {{
                        {access} const byte Ok = 0;
                        {access} const byte Err = 1;
                    }}
                }}
                "# ,) } if self . needs_option { uwrite ! (src , r#"

                {access} class Option<T> {{
                    private static Option<T> none = new ();

                    private Option()
                    {{
                        HasValue = false;
                    }}

                    {access} Option(T v)
                    {{
                        HasValue = true;
                        Value = v;
                    }}

                    {access} static Option<T> None => none;

                    [global::System.Diagnostics.CodeAnalysis.MemberNotNullWhenAttribute(true, nameof(Value))]
                    {access} bool HasValue {{ get; }}

                    {access} T? Value {{ get; }}
                }}
                "# ,) } if self . needs_wit_exception { uwrite ! (src , r#"
                {access} class WitException: global::System.Exception {{
                    {access} object Value {{ get; }}
                    {access} uint NestingLevel {{ get; }}

                    {access} WitException(object v, uint level)
                    {{
                        Value = v;
                        NestingLevel = level;
                    }}
                }}

                {access} class WitException<T>: WitException {{
                    {access} T TypedValue {{ get {{ return (T)this.Value;}} }}

                    {access} WitException(T v, uint level) : base(v!, level)
                    {{
                    }}
                }}
                "# ,) } if self . needs_export_return_area { let mut ret_area_str = String :: new () ; let (array_size , element_type) = dotnet_aligned_array (self . return_area_size , self . return_area_align) ; uwrite ! (ret_area_str , "
                {access} static class InteropReturnArea
                {{
                    [global::System.Runtime.CompilerServices.InlineArrayAttribute({0})]
                    [global::System.Runtime.InteropServices.StructLayoutAttribute(global::System.Runtime.InteropServices.LayoutKind.Sequential, Pack = {1})]
                    internal struct ReturnArea
                    {{
                        private {2} buffer;

                        internal unsafe nint AddressOfReturnArea()
                        {{
                            return (nint)global::System.Runtime.CompilerServices.Unsafe.AsPointer(ref buffer);
                        }}
                    }}

                    [global::System.ThreadStaticAttribute]
                    [global::System.Runtime.CompilerServices.FixedAddressValueTypeAttribute]
                    internal static ReturnArea returnArea = default;
                }}
                " , array_size , self . return_area_align , element_type) ; src . push_str (& ret_area_str) ; } if self . needs_rep_table { src . push_str ("\n") ; src . push_str (include_str ! ("RepTable.cs")) ; } if ! & self . world_fragments . is_empty () { src . push_str ("\n") ; src . push_str ("namespace exports {\n") ; src . push_str (& format ! ("{access} static class {name}World\n")) ; src . push_str ("{") ; for fragment in & self . world_fragments { src . push_str ("\n") ; src . push_str (& fragment . csharp_interop_src) ; } src . push_str ("}\n") ; src . push_str ("}\n") ; } if self . needs_async_support { src . push_str ("\n") ; src . push_str (include_str ! ("AsyncSupport.cs")) ; } src . push_str ("\n") ; src . push_str ("}\n") ; files . push (& format ! ("{name}.cs") , indent (& src) . as_bytes ()) ; let generate_stub = | name : String , files : & mut Files , stubs : Stubs | { let (stub_namespace , interface_or_class_name) = CSharp :: get_class_name_from_qualified_name (& name) ; let stub_class_name = format ! ("{}Impl" , match interface_or_class_name . starts_with ("I") { true => interface_or_class_name . strip_prefix ("I") . unwrap () . to_string () , false => interface_or_class_name . clone () , }) ; let stub_file_name = match stub_namespace . len () { 0 => stub_class_name . clone () , _ => format ! ("{stub_namespace}.{stub_class_name}") , } ; let (fragments , fully_qualified_namespace) = match stubs { Stubs :: World (fragments) => { let fully_qualified_namespace = format ! ("{namespace}") ; (fragments , fully_qualified_namespace) } Stubs :: Interface (fragments) => { let fully_qualified_namespace = format ! ("{stub_namespace}") ; (fragments , fully_qualified_namespace) } } ; if fragments . iter () . all (| f | f . stub . is_empty ()) { return ; } let body = fragments . iter () . map (| f | f . stub . deref ()) . collect :: < Vec < _ > > () . join ("\n") ; let body = format ! ("{header}
                 namespace {fully_qualified_namespace};

                 {access} partial class {stub_class_name} : {interface_or_class_name} {{
                    {body}
                 }}
                ") ; files . push (& format ! ("{stub_file_name}.cs") , indent (& body) . as_bytes ()) ; } ; if self . opts . generate_stub { generate_stub (format ! ("I{name}World") , files , Stubs :: World (& self . world_fragments) ,) ; } if ! self . opts . skip_support_files { if self . opts . runtime == CSharpRuntime :: Mono { files . push (& "MonoEntrypoint.cs" . to_string () , indent (& format ! (r#"
                        {access} class MonoEntrypoint() {{
                            {access} static void Main() {{
                            }}
                        }}
                        "#)) . as_bytes () ,) ; } { let (resolve , world) = wit_parser :: decoding :: decode_world (& wit_component :: metadata :: encode (& resolve , id , self . opts . string_encoding , None ,) ?) ? ; let pkg = resolve . worlds [world] . package . unwrap () ; let mut printer = WitPrinter :: default () ; printer . emit_docs (false) ; printer . print (& resolve , pkg , & resolve . packages . iter () . filter_map (| (id , _) | if id == pkg { None } else { Some (id) }) . collect :: < Vec < _ > > () ,) ? ; files . push (& format ! ("{world_namespace}_component_type.wit") , String :: from (printer . output) . as_bytes () ,) ; } let mut wasm_import_linakge_src = String :: new () ; uwrite ! (wasm_import_linakge_src , r#"{header}
                #if !NET9_0_OR_GREATER
                // temporarily add this attribute until it is available in dotnet 9
                namespace System.Runtime.InteropServices
                {{
                    internal partial class WasmImportLinkageAttribute : global::System.Attribute {{}}
                }}
                #endif
                "# ,) ; files . push (& format ! ("{world_namespace}_wasm_import_linkage_attribute.cs") , indent (& wasm_import_linakge_src) . as_bytes () ,) ; } for (full_name , interface_type_and_fragments) in & self . interface_fragments { let fragments = & interface_type_and_fragments . interface_fragments ; let (namespace , interface_name) = & CSharp :: get_class_name_from_qualified_name (full_name) ; let body = fragments . iter () . map (| f | f . csharp_src . deref ()) . collect :: < Vec < _ > > () . join ("\n") ; if body . len () > 0 { let body = format ! ("{header}

                    namespace {namespace};

                    {access} interface {interface_name} {{
                        {body}
                    }}
                    " ,) ; files . push (& format ! ("{full_name}.cs") , indent (& body) . as_bytes ()) ; } let body = fragments . iter () . map (| f | f . csharp_interop_src . deref ()) . collect :: < Vec < _ > > () . join ("\n") ; let class_name = interface_name . strip_prefix ("I") . unwrap () ; let body = format ! ("{header}

                namespace {namespace}
                {{
                  {access} static class {class_name}Interop {{
                      {body}
                  }}
                }}
                " ,) ; files . push (& format ! ("{namespace}.{class_name}Interop.cs") , indent (& body) . as_bytes () ,) ; if interface_type_and_fragments . is_export && self . opts . generate_stub { generate_stub (full_name . to_string () , files , Stubs :: Interface (fragments)) ; } } Ok (()) } }
};
}
