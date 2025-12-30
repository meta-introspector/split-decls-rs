// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl WorldGenerator for C { fn preprocess (& mut self , resolve : & Resolve , world : WorldId) { self . world = self . opts . rename_world . clone () . unwrap_or_else (| | resolve . worlds [world] . name . clone ()) ; self . sizes . fill (resolve) ; self . world_id = Some (world) ; let mut interfaces = HashMap :: new () ; let world = & resolve . worlds [world] ; for (key , _item) in world . imports . iter () . chain (world . exports . iter ()) { let name = resolve . name_world_key (key) ; interfaces . insert (name , key . clone ()) ; } for (from , to) in self . opts . rename . iter () { match interfaces . get (from) { Some (key) => { self . renamed_interfaces . insert (key . clone () , to . clone ()) ; } None => { eprintln ! ("warning: rename of `{from}` did not match any interfaces") ; } } } } fn import_interface (& mut self , resolve : & Resolve , name : & WorldKey , id : InterfaceId , _files : & mut Files ,) -> Result < () > { let wasm_import_module = resolve . name_world_key (name) ; let mut r#gen = self . interface (resolve , true , Some (& wasm_import_module)) ; r#gen . interface = Some ((id , name)) ; r#gen . define_interface_types (id) ; for (i , (_name , func)) in resolve . interfaces [id] . functions . iter () . enumerate () { if i == 0 { let name = resolve . name_world_key (name) ; uwriteln ! (r#gen . src . h_fns , "\n// Imported Functions from `{name}`") ; uwriteln ! (r#gen . src . c_fns , "\n// Imported Functions from `{name}`") ; } r#gen . import (Some (name) , func) ; } r#gen . r#gen . src . append (& r#gen . src) ; Ok (()) } fn import_funcs (& mut self , resolve : & Resolve , world : WorldId , funcs : & [(& str , & Function)] , _files : & mut Files ,) { let name = & resolve . worlds [world] . name ; let mut r#gen = self . interface (resolve , true , Some ("$root")) ; r#gen . define_function_types (funcs) ; for (i , (_name , func)) in funcs . iter () . enumerate () { if i == 0 { uwriteln ! (r#gen . src . h_fns , "\n// Imported Functions from `{name}`") ; uwriteln ! (r#gen . src . c_fns , "\n// Imported Functions from `{name}`") ; } r#gen . import (None , func) ; } r#gen . r#gen . src . append (& r#gen . src) ; } fn export_interface (& mut self , resolve : & Resolve , name : & WorldKey , id : InterfaceId , _files : & mut Files ,) -> Result < () > { let mut r#gen = self . interface (resolve , false , None) ; r#gen . interface = Some ((id , name)) ; r#gen . define_interface_types (id) ; for (i , (_name , func)) in resolve . interfaces [id] . functions . iter () . enumerate () { if i == 0 { let name = resolve . name_world_key (name) ; uwriteln ! (r#gen . src . h_fns , "\n// Exported Functions from `{name}`") ; uwriteln ! (r#gen . src . c_fns , "\n// Exported Functions from `{name}`") ; } r#gen . export (func , Some (name)) ; } r#gen . r#gen . src . append (& r#gen . src) ; Ok (()) } fn export_funcs (& mut self , resolve : & Resolve , world : WorldId , funcs : & [(& str , & Function)] , _files : & mut Files ,) -> Result < () > { let name = & resolve . worlds [world] . name ; let mut r#gen = self . interface (resolve , false , None) ; r#gen . define_function_types (funcs) ; for (i , (_name , func)) in funcs . iter () . enumerate () { if i == 0 { uwriteln ! (r#gen . src . h_fns , "\n// Exported Functions from `{name}`") ; uwriteln ! (r#gen . src . c_fns , "\n// Exported Functions from `{name}`") ; } r#gen . export (func , None) ; } r#gen . r#gen . src . append (& r#gen . src) ; Ok (()) } fn import_types (& mut self , resolve : & Resolve , _world : WorldId , types : & [(& str , TypeId)] , _files : & mut Files ,) { let mut r#gen = self . interface (resolve , true , Some ("$root")) ; let mut live = LiveTypes :: default () ; for (_ , id) in types { live . add_type_id (resolve , * id) ; } r#gen . define_live_types (live) ; r#gen . r#gen . src . append (& r#gen . src) ; } fn finish (& mut self , resolve : & Resolve , id : WorldId , files : & mut Files) -> Result < () > { let linking_symbol = component_type_object :: linking_symbol (& self . world) ; self . c_include ("<stdlib.h>") ; let snake = self . world . to_snake_case () ; uwriteln ! (self . src . c_adapters , "\n// Ensure that the *_component_type.o object is linked in") ; uwrite ! (self . src . c_adapters , "
               extern void {linking_symbol}(void);
               __attribute__((used))
               void {linking_symbol}_public_use_in_this_compilation_unit(void) {{
                   {linking_symbol}();
               }}
           " ,) ; self . print_intrinsics () ; if self . needs_string { self . c_include ("<string.h>") ; let (strlen , size) = match self . opts . string_encoding { StringEncoding :: UTF8 => (format ! ("strlen(s)") , 1) , StringEncoding :: UTF16 => { self . h_include ("<uchar.h>") ; uwrite ! (self . src . h_helpers , "
                            // Returns the length of the UTF-16 string `s` in code units
                            size_t {snake}_string_len(const char16_t* s);
                        " ,) ; uwrite ! (self . src . c_helpers , "
                            size_t {snake}_string_len(const char16_t* s) {{
                                char16_t* c = (char16_t*)s;
                                for (; *c; ++c);
                                return c-s;
                            }}
                        " ,) ; (format ! ("{snake}_string_len(s)") , 2) } StringEncoding :: CompactUTF16 => unimplemented ! () , } ; let ty = self . char_type () ; let c_string_ty = match self . opts . string_encoding { StringEncoding :: UTF8 => "char" , StringEncoding :: UTF16 => "char16_t" , StringEncoding :: CompactUTF16 => panic ! ("Compact UTF16 unsupported") , } ; uwrite ! (self . src . h_helpers , "
                   // Sets the string `ret` to reference the input string `s` without copying it
                   void {snake}_string_set({snake}_string_t *ret, const {c_string_ty} *s);

                   // Creates a copy of the input nul-terminated string `s` and
                   // stores it into the component model string `ret`.
                   void {snake}_string_dup({snake}_string_t *ret, const {c_string_ty} *s);

                   // Deallocates the string pointed to by `ret`, deallocating
                   // the memory behind the string.
                   void {snake}_string_free({snake}_string_t *ret);\
               " ,) ; uwrite ! (self . src . c_helpers , "
                   void {snake}_string_set({snake}_string_t *ret, const {c_string_ty} *s) {{
                       ret->ptr = ({ty}*) s;
                       ret->len = {strlen};
                   }}

                   void {snake}_string_dup({snake}_string_t *ret, const {c_string_ty} *s) {{
                       ret->len = {strlen};
                       ret->ptr = ({ty}*) cabi_realloc(NULL, 0, {size}, ret->len * {size});
                       memcpy(ret->ptr, s, ret->len * {size});
                   }}

                   void {snake}_string_free({snake}_string_t *ret) {{
                       if (ret->len > 0) {{
                           free(ret->ptr);
                       }}
                       ret->ptr = NULL;
                       ret->len = 0;
                   }}
               " ,) ; } if self . needs_union_int32_float { uwriteln ! (self . src . c_defs , "\nunion int32_float {{ int32_t a; float b; }};") ; } if self . needs_union_float_int32 { uwriteln ! (self . src . c_defs , "\nunion float_int32 {{ float a; int32_t b; }};") ; } if self . needs_union_int64_double { uwriteln ! (self . src . c_defs , "\nunion int64_double {{ int64_t a; double b; }};") ; } if self . needs_union_double_int64 { uwriteln ! (self . src . c_defs , "\nunion double_int64 {{ double a; int64_t b; }};") ; } if self . needs_async || self . futures . len () > 0 { self . generate_async_helpers () ; } let version = env ! ("CARGO_PKG_VERSION") ; let mut h_str = wit_bindgen_core :: Source :: default () ; wit_bindgen_core :: generated_preamble (& mut h_str , version) ; uwrite ! (h_str , "#ifndef __BINDINGS_{0}_H
            #define __BINDINGS_{0}_H
            #ifdef __cplusplus
            extern \"C\" {{" , self . world . to_shouty_snake_case () ,) ; h_str . deindent (1) ; uwriteln ! (h_str , "\n#endif\n") ; uwriteln ! (h_str , "#include <stdint.h>") ; uwriteln ! (h_str , "#include <stdbool.h>") ; uwriteln ! (h_str , "#include <stddef.h>") ; for include in self . h_includes . iter () { uwriteln ! (h_str , "#include {include}") ; } let mut c_str = wit_bindgen_core :: Source :: default () ; wit_bindgen_core :: generated_preamble (& mut c_str , version) ; uwriteln ! (c_str , "#include \"{snake}.h\"") ; for include in self . c_includes . iter () { uwriteln ! (c_str , "#include {include}") ; } c_str . push_str (& self . src . c_defs) ; c_str . push_str (& self . src . c_fns) ; if ! self . return_pointer_area_size . is_empty () { uwrite ! (c_str , "
                __attribute__((__aligned__({})))
                static uint8_t RET_AREA[{}];
                " , self . return_pointer_area_align . format (POINTER_SIZE_EXPRESSION) , self . return_pointer_area_size . format (POINTER_SIZE_EXPRESSION) ,) ; } if self . needs_string { uwriteln ! (h_str , "
                typedef struct {snake}_string_t {{\n\
                  {ty} *ptr;\n\
                  size_t len;\n\
                }} {snake}_string_t;" , ty = self . char_type () ,) ; } if self . src . h_async . len () > 0 { uwriteln ! (h_str , "\n// Async Helper Functions") ; h_str . push_str (& self . src . h_async) ; h_str . push_str ("\n") ; } if self . src . h_defs . len () > 0 { h_str . push_str (& self . src . h_defs) ; } h_str . push_str (& self . src . h_fns) ; if ! self . opts . no_helpers && self . src . h_helpers . len () > 0 { uwriteln ! (h_str , "\n// Helper Functions") ; h_str . push_str (& self . src . h_helpers) ; h_str . push_str ("\n") ; } if ! self . opts . no_helpers && self . src . c_helpers . len () > 0 { uwriteln ! (c_str , "\n// Helper Functions") ; c_str . push_str (self . src . c_helpers . as_mut_string ()) ; } if self . src . c_async . len () > 0 { uwriteln ! (c_str , "\n// Async Helper Functions") ; c_str . push_str (& self . src . c_async) ; c_str . push_str ("\n") ; } uwriteln ! (c_str , "\n// Component Adapters") ; c_str . push_str (& self . src . c_adapters) ; uwriteln ! (h_str , "
            #ifdef __cplusplus
            }}
            #endif
            #endif") ; files . push (& format ! ("{snake}.h") , h_str . as_bytes ()) ; files . push (& format ! ("{snake}.c") , c_str . as_bytes ()) ; if ! self . opts . no_object_file { files . push (& format ! ("{snake}_component_type.o" ,) , component_type_object :: object (resolve , id , & self . world , self . opts . string_encoding , self . opts . type_section_suffix . as_deref () ,) . unwrap () . as_slice () ,) ; } Ok (()) } fn pre_export_interface (& mut self , resolve : & Resolve , _files : & mut Files) -> Result < () > { self . remove_types_redefined_by_exports (resolve , self . world_id . unwrap ()) ; Ok (()) } }
};
}
