// Generated macro for impl_58 (impl)
macro_rules! Depcrate_interfaceimpl_58 {
() => {
// Module: crate::interface
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a > CoreInterfaceGenerator < 'a > for InterfaceGenerator < 'a > { fn resolve (& self) -> & 'a Resolve { self . resolve } fn type_record (& mut self , _id : TypeId , name : & str , record : & Record , docs : & Docs) { let access = self . csharp_gen . access_modifier () ; self . print_docs (docs) ; let name = name . to_upper_camel_case () ; let parameters = record . fields . iter () . map (| field | { format ! ("{} {}" , self . type_name (& field . ty) , field . name . to_csharp_ident ()) }) . collect :: < Vec < _ > > () . join (", ") ; let assignments = record . fields . iter () . map (| field | { let name = field . name . to_csharp_ident () ; format ! ("this.{name} = {name};") }) . collect :: < Vec < _ > > () . join ("\n") ; let fields = if record . fields . is_empty () { format ! ("{access} const {name} INSTANCE = new {name}();") } else { record . fields . iter () . map (| field | { format ! ("{access} readonly {} {};" , self . type_name (& field . ty) , field . name . to_csharp_ident ()) }) . collect :: < Vec < _ > > () . join ("\n") } ; uwrite ! (self . src , "
            {access} readonly struct {name} {{
                {fields}

                {access} {name}({parameters}) {{
                    {assignments}
                }}
            }}
            ") ; } fn type_flags (& mut self , _id : TypeId , name : & str , flags : & Flags , docs : & Docs) { self . print_docs (docs) ; let name = name . to_upper_camel_case () ; let enum_elements = flags . flags . iter () . enumerate () . map (| (i , flag) | { let flag_name = flag . name . to_shouty_snake_case () ; let suffix = if matches ! (flags . repr () , FlagsRepr :: U32 (2)) { "UL" } else { "" } ; format ! ("{flag_name} = 1{suffix} << {i},") }) . collect :: < Vec < _ > > () . join ("\n") ; let enum_type = match flags . repr () { FlagsRepr :: U32 (2) => ": ulong" , FlagsRepr :: U16 => ": ushort" , FlagsRepr :: U8 => ": byte" , _ => "" , } ; let access = self . csharp_gen . access_modifier () ; uwrite ! (self . src , "
            {access} enum {name} {enum_type} {{
                {enum_elements}
            }}
            ") ; } fn type_tuple (& mut self , id : TypeId , _name : & str , _tuple : & Tuple , _docs : & Docs) { self . type_name (& Type :: Id (id)) ; } fn type_variant (& mut self , _id : TypeId , name : & str , variant : & Variant , docs : & Docs) { self . print_docs (docs) ; let name = name . to_upper_camel_case () ; let tag_type = int_type (variant . tag ()) ; let access = self . csharp_gen . access_modifier () ; let constructors = variant . cases . iter () . map (| case | { let case_name = case . name . to_csharp_ident () ; let tag = case . name . to_csharp_ident_upper () ; let (parameter , argument) = if let Some (ty) = self . non_empty_type (case . ty . as_ref ()) { (format ! ("{} {case_name}" , self . type_name (ty)) , case_name . deref () ,) } else { (String :: new () , "null") } ; format ! ("{access} static {name} {tag}({parameter}) {{
                         return new {name}(Tags.{tag}, {argument});
                     }}
                    ") }) . collect :: < Vec < _ > > () . join ("\n") ; let accessors = variant . cases . iter () . filter_map (| case | { self . non_empty_type (case . ty . as_ref ()) . map (| ty | { let case_name = case . name . to_upper_camel_case () ; let tag = case . name . to_csharp_ident_upper () ; let ty = self . type_name (ty) ; format ! (r#"{access} {ty} As{case_name}
                        {{
                            get
                            {{
                                if (Tag == Tags.{tag})
                                    return ({ty})value!;
                                else
                                    throw new global::System.ArgumentException("expected {tag}, got " + Tag);
                            }}
                        }}
                        "#) }) }) . collect :: < Vec < _ > > () . join ("\n") ; let tags = variant . cases . iter () . enumerate () . map (| (i , case) | { let tag = case . name . to_csharp_ident_upper () ; format ! ("{access} const {tag_type} {tag} = {i};") }) . collect :: < Vec < _ > > () . join ("\n") ; uwrite ! (self . src , "
            {access} class {name} {{
                {access} readonly {tag_type} Tag;
                private readonly object? value;

                private {name}({tag_type} tag, object? value) {{
                    this.Tag = tag;
                    this.value = value;
                }}

                {constructors}
                {accessors}

                {access} class Tags {{
                    {tags}
                }}
            }}
            ") ; } fn type_option (& mut self , id : TypeId , _name : & str , _payload : & Type , _docs : & Docs) { self . type_name (& Type :: Id (id)) ; } fn type_result (& mut self , id : TypeId , _name : & str , _result : & Result_ , _docs : & Docs) { self . type_name (& Type :: Id (id)) ; } fn type_enum (& mut self , _id : TypeId , name : & str , enum_ : & Enum , docs : & Docs) { self . print_docs (docs) ; let name = name . to_upper_camel_case () ; let cases = enum_ . cases . iter () . map (| case | case . name . to_shouty_snake_case ()) . collect :: < Vec < _ > > () . join (", ") ; let access = self . csharp_gen . access_modifier () ; uwrite ! (self . src , "
            {access} enum {name} {{
                {cases}
            }}
            ") ; } fn type_alias (& mut self , id : TypeId , _name : & str , _ty : & Type , _docs : & Docs) { self . type_name (& Type :: Id (id)) ; } fn type_list (& mut self , id : TypeId , _name : & str , _ty : & Type , _docs : & Docs) { self . type_name (& Type :: Id (id)) ; } fn type_builtin (& mut self , _id : TypeId , _name : & str , _ty : & Type , _docs : & Docs) { unimplemented ! () ; } fn type_resource (& mut self , id : TypeId , name : & str , docs : & Docs) { self . csharp_gen . all_resources . entry (id) . or_insert_with (| | ResourceInfo { module : self . name . to_owned () , name : name . to_owned () , docs : docs . clone () , direction : Direction :: Import , }) . direction = self . direction ; } fn type_future (& mut self , id : TypeId , _name : & str , _ty : & Option < Type > , _docs : & Docs) { self . type_name (& Type :: Id (id)) ; } fn type_stream (& mut self , id : TypeId , _name : & str , _ty : & Option < Type > , _docs : & Docs) { self . type_name (& Type :: Id (id)) ; } }
};
}
