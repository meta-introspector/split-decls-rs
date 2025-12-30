// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a > wit_bindgen_core :: InterfaceGenerator < 'a > for InterfaceGenerator < 'a > { fn resolve (& self) -> & 'a Resolve { self . resolve } fn type_record (& mut self , _id : TypeId , name : & str , record : & Record , docs : & Docs) { print_docs (& mut self . src , docs) ; let name = name . to_moonbit_type_ident () ; let parameters = record . fields . iter () . map (| field | { format ! ("{} : {}" , field . name . to_moonbit_ident () , self . type_name (& field . ty , true) ,) }) . collect :: < Vec < _ > > () . join ("; ") ; let mut deriviation : Vec < _ > = Vec :: new () ; if self . gen . opts . derive_show { deriviation . push ("Show") } if self . gen . opts . derive_eq { deriviation . push ("Eq") } uwrite ! (self . src , "
            pub(all) struct {name} {{
                {parameters}
            }} derive({})
            " , deriviation . join (", ")) ; } fn type_resource (& mut self , _id : TypeId , name : & str , docs : & Docs) { print_docs (& mut self . src , docs) ; let type_name = name ; let name = name . to_moonbit_type_ident () ; let mut deriviation : Vec < _ > = Vec :: new () ; if self . gen . opts . derive_show { deriviation . push ("Show") } if self . gen . opts . derive_eq { deriviation . push ("Eq") } let declaration = if self . gen . opts . derive_error && name . contains ("Error") { "suberror" } else { "struct" } ; uwrite ! (self . src , r#"
            pub(all) {declaration} {name}(Int) derive({})
            "# , deriviation . join (", ") ,) ; let module = self . module ; if self . direction == Direction :: Import { uwrite ! (& mut self . src , r#"
                /// Drops a resource handle.
                pub fn {name}::drop(self : {name}) -> Unit {{
                    let {name}(resource) = self
                    wasmImportResourceDrop{name}(resource)
                }}
                "# ,) ; uwrite ! (& mut self . ffi , r#"
                fn wasmImportResourceDrop{name}(resource : Int) = "{module}" "[resource-drop]{type_name}"
                "# ,) } else { uwrite ! (& mut self . src , r#"
                /// Creates a new resource with the given `rep` as its representation and returning the handle to this resource.
                pub fn {name}::new(rep : Int) -> {name} {{
                    {name}::{name}(wasmExportResourceNew{name}(rep))
                }}
                fn wasmExportResourceNew{name}(rep : Int) -> Int = "[export]{module}" "[resource-new]{type_name}"

                /// Drops a resource handle.
                pub fn {name}::drop(self : Self) -> Unit {{
                    let {name}(resource) = self
                    wasmExportResourceDrop{name}(resource)
                }}
                fn wasmExportResourceDrop{name}(resource : Int) = "[export]{module}" "[resource-drop]{type_name}"

                /// Gets the `Int` representation of the resource pointed to the given handle.
                pub fn {name}::rep(self : Self) -> Int {{
                    let {name}(resource) = self
                    wasmExportResourceRep{name}(resource)
                }}
                fn wasmExportResourceRep{name}(resource : Int) -> Int = "[export]{module}" "[resource-rep]{type_name}"
                "# ,) ; uwrite ! (& mut self . stub , r#"
                /// Destructor of the resource.
                pub fn {name}::dtor(_self : {name}) -> Unit {{
                  ...
                }}
                "#) ; let func_name = self . gen . export_ns . tmp (& format ! ("wasmExport{name}Dtor")) ; let export_dir = self . gen . opts . gen_dir . clone () ; let mut gen = self . gen . interface (self . resolve , export_dir . as_str () , "" , Direction :: Export) ; uwrite ! (self . ffi , r#"
                pub fn {func_name}(handle : Int) -> Unit {{
                    {}{name}::dtor(handle)
                }}
                "# , gen . qualify_package (self . name)) ; self . gen . export . insert (func_name , format ! ("{module}#[dtor]{type_name}")) ; } } fn type_flags (& mut self , _id : TypeId , name : & str , flags : & Flags , docs : & Docs) { print_docs (& mut self . src , docs) ; let name = name . to_moonbit_type_ident () ; let ty = match flags . repr () { FlagsRepr :: U8 => "Byte" , FlagsRepr :: U16 | FlagsRepr :: U32 (1) => "UInt" , FlagsRepr :: U32 (2) => "UInt64" , _ => unreachable ! () , } ; let cases = flags . flags . iter () . map (| flag | flag . name . to_shouty_snake_case ()) . collect :: < Vec < _ > > () . join ("; ") ; let map_to_int = flags . flags . iter () . enumerate () . map (| (i , flag) | { let flag_name = flag . name . to_shouty_snake_case () ; let suffix = if matches ! (flags . repr () , FlagsRepr :: U32 (2)) { "UL" } else { "U" } ; let cast = if matches ! (flags . repr () , FlagsRepr :: U8) { ".to_byte()" } else { "" } ; format ! ("{flag_name} => ((1{suffix} << {i}){cast})") }) . collect :: < Vec < _ > > () . join ("\n    ") ; let mut deriviation : Vec < _ > = Vec :: new () ; if self . gen . opts . derive_show { deriviation . push ("Show") } if self . gen . opts . derive_eq { deriviation . push ("Eq") } let declaration = if self . gen . opts . derive_error && name . contains ("Error") { "suberror" } else { "struct" } ; uwrite ! (self . src , "
            pub(all) {declaration} {name}({ty}) derive({})
            pub fn {name}::default() -> {name} {{
                {}
            }}
            pub(all) enum {name}Flag {{
                {cases}
            }}
            fn {name}Flag::value(self : {name}Flag) -> {ty} {{
              match self {{
                {map_to_int}
              }}
            }}
            pub fn {name}::set(self : Self, other: {name}Flag) -> {name} {{
              let {name}(flag) = self
              flag.lor(other.value())
            }}
            pub fn {name}::unset(self : Self, other: {name}Flag) -> {name} {{
              let {name}(flag) = self
              flag.land(other.value().lnot())
            }}
            pub fn {name}::is_set(self : Self, other: {name}Flag) -> Bool {{
              let {name}(flag) = self
              (flag.land(other.value()) == other.value())
            }}
            " , deriviation . join (", ") , match ty { "Byte" => "b'\\x00'" , "UInt" => "0U" , "UInt64" => "0UL" , _ => unreachable ! () , }) ; } fn type_tuple (& mut self , _id : TypeId , _name : & str , _tuple : & Tuple , _docs : & Docs) { } fn type_variant (& mut self , _id : TypeId , name : & str , variant : & Variant , docs : & Docs) { print_docs (& mut self . src , docs) ; let name = name . to_moonbit_type_ident () ; let cases = variant . cases . iter () . map (| case | { let name = case . name . to_upper_camel_case () ; if let Some (ty) = case . ty { let ty = self . type_name (& ty , true) ; format ! ("{name}({ty})") } else { name . to_string () } }) . collect :: < Vec < _ > > () . join ("\n  ") ; let mut deriviation : Vec < _ > = Vec :: new () ; if self . gen . opts . derive_show { deriviation . push ("Show") } if self . gen . opts . derive_eq { deriviation . push ("Eq") } let declaration = if self . gen . opts . derive_error && name . contains ("Error") { "suberror" } else { "enum" } ; uwrite ! (self . src , "
            pub(all) {declaration} {name} {{
              {cases}
            }} derive({})
            " , deriviation . join (", ")) ; } fn type_option (& mut self , _id : TypeId , _name : & str , _payload : & Type , _docs : & Docs) { } fn type_result (& mut self , _id : TypeId , _name : & str , _result : & Result_ , _docs : & Docs) { } fn type_enum (& mut self , _id : TypeId , name : & str , enum_ : & Enum , docs : & Docs) { print_docs (& mut self . src , docs) ; let name = name . to_moonbit_type_ident () ; let cases = enum_ . cases . iter () . map (| case | case . name . to_shouty_snake_case ()) . collect :: < Vec < _ > > () . join ("; ") ; let mut deriviation : Vec < _ > = Vec :: new () ; if self . gen . opts . derive_show { deriviation . push ("Show") } if self . gen . opts . derive_eq { deriviation . push ("Eq") } let declaration = if self . gen . opts . derive_error && name . contains ("Error") { "suberror" } else { "enum" } ; uwrite ! (self . src , "
            pub(all) {declaration} {name} {{
                {cases}
            }} derive({})
            " , deriviation . join (", ")) ; let cases = enum_ . cases . iter () . enumerate () . map (| (i , case) | format ! ("{} => {i}" , case . name . to_shouty_snake_case ())) . collect :: < Vec < _ > > () . join ("\n  ") ; uwrite ! (self . src , "
            pub fn {name}::ordinal(self : {name}) -> Int {{
              match self {{
                {cases}
              }}
            }}
            ") ; let cases = enum_ . cases . iter () . enumerate () . map (| (i , case) | format ! ("{i} => {}" , case . name . to_shouty_snake_case ())) . collect :: < Vec < _ > > () . join ("\n  ") ; uwrite ! (self . src , "
            pub fn {name}::from(self : Int) -> {name} {{
              match self {{
                {cases}
                _ => panic()
              }}
            }}
            ") ; } fn type_alias (& mut self , _id : TypeId , _name : & str , _ty : & Type , _docs : & Docs) { } fn type_list (& mut self , _id : TypeId , _name : & str , _ty : & Type , _docs : & Docs) { } fn type_future (& mut self , _id : TypeId , _name : & str , _ty : & Option < Type > , _docs : & Docs) { unimplemented ! () } fn type_stream (& mut self , _id : TypeId , _name : & str , _ty : & Option < Type > , _docs : & Docs) { unimplemented ! () } fn type_builtin (& mut self , _id : TypeId , _name : & str , _ty : & Type , _docs : & Docs) { unimplemented ! () ; } }
};
}
