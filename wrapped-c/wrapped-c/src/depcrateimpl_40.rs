// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a > wit_bindgen_core :: InterfaceGenerator < 'a > for InterfaceGenerator < 'a > { fn resolve (& self) -> & 'a Resolve { self . resolve } fn type_record (& mut self , id : TypeId , _name : & str , record : & Record , docs : & Docs) { self . src . h_defs ("\n") ; self . docs (docs , SourceType :: HDefs) ; self . start_typedef_struct (id) ; for field in record . fields . iter () { self . docs (& field . docs , SourceType :: HDefs) ; self . print_ty (SourceType :: HDefs , & field . ty) ; self . src . h_defs (" ") ; self . src . h_defs (& to_c_ident (& field . name)) ; self . src . h_defs (";\n") ; } self . finish_typedef_struct (id) ; } fn type_resource (& mut self , id : TypeId , name : & str , _docs : & Docs) { let ns = self . owner_namespace (id) ; let snake = name . to_snake_case () ; let mut own = ns . clone () ; let mut borrow = own . clone () ; own . push_str ("_own") ; borrow . push_str ("_borrow") ; own . push_str ("_") ; borrow . push_str ("_") ; own . push_str (& snake) ; borrow . push_str (& snake) ; own . push_str ("_t") ; borrow . push_str ("_t") ; self . src . h_helpers (& format ! ("
extern void {ns}_{snake}_drop_own({own} handle);
            ")) ; let import_module = if self . in_import { self . wasm_import_module . unwrap () . to_string () } else { let module = match self . interface { Some ((_ , key)) => self . resolve . name_world_key (key) , None => unimplemented ! ("resource exports from worlds") , } ; format ! ("[export]{module}") } ; let drop_fn = format ! ("__wasm_import_{ns}_{snake}_drop") ; self . src . c_helpers (& format ! (r#"
__attribute__((__import_module__("{import_module}"), __import_name__("[resource-drop]{name}")))
extern void {drop_fn}(int32_t handle);

void {ns}_{snake}_drop_own({own} handle) {{
    {drop_fn}(handle.__handle);
}}
            "#)) ; self . src . h_defs (& format ! ("\ntypedef struct {own} {{\nint32_t __handle;\n}} {own};\n")) ; if self . in_import { self . src . h_defs (& format ! ("\ntypedef struct {borrow} {{\nint32_t __handle;\n}} {borrow};\n")) ; if ! self . autodrop_enabled () { self . src . h_helpers (& format ! ("\nextern void {ns}_{snake}_drop_borrow({borrow} handle);\n")) ; self . src . c_helpers (& format ! ("
void {ns}_{snake}_drop_borrow({borrow} handle) {{
    __wasm_import_{ns}_{snake}_drop(handle.__handle);
}}
                ")) ; } self . src . h_helpers (& format ! ("
extern {borrow} {ns}_borrow_{snake}({own} handle);
                ")) ; self . src . c_helpers (& format ! (r#"
{borrow} {ns}_borrow_{snake}({own} arg) {{
    return ({borrow}) {{ arg.__handle }};
}}
                "#)) ; } else { self . src . h_defs ("\n") ; self . src . h_defs ("typedef struct ") ; let ty_name = self . r#gen . type_names [& id] . clone () ; self . src . h_defs (& ty_name) ; self . src . h_defs (" ") ; self . print_typedef_target (id) ; let (_ , key) = self . interface . unwrap () ; let module = self . resolve . name_world_key (key) ; self . src . h_defs (& format ! ("\ntypedef {ty_name}* {borrow};\n")) ; self . src . h_helpers (& format ! ("
extern {own} {ns}_{snake}_new({ty_name} *rep);
extern {ty_name}* {ns}_{snake}_rep({own} handle);
void {ns}_{snake}_destructor({ty_name} *rep);
                ")) ; self . src . c_helpers (& format ! (r#"
__attribute__(( __import_module__("[export]{module}"), __import_name__("[resource-new]{name}")))
extern int32_t __wasm_import_{ns}_{snake}_new(int32_t);

__attribute__((__import_module__("[export]{module}"), __import_name__("[resource-rep]{name}")))
extern int32_t __wasm_import_{ns}_{snake}_rep(int32_t);

{own} {ns}_{snake}_new({ty_name} *rep) {{
    return ({own}) {{ __wasm_import_{ns}_{snake}_new((int32_t) rep) }};
}}

{ty_name}* {ns}_{snake}_rep({own} handle) {{
    return ({ns}_{snake}_t*) __wasm_import_{ns}_{snake}_rep(handle.__handle);
}}

__attribute__((__export_name__("{module}#[dtor]{snake}")))
void __wasm_export_{ns}_{snake}_dtor({ns}_{snake}_t* arg) {{
    {ns}_{snake}_destructor(arg);
}}
                "#)) ; } self . r#gen . resources . insert (id , ResourceInfo { own , borrow , direction : if self . in_import { Direction :: Import } else { Direction :: Export } , drop_fn , } ,) ; } fn type_tuple (& mut self , id : TypeId , _name : & str , tuple : & Tuple , docs : & Docs) { self . src . h_defs ("\n") ; self . docs (docs , SourceType :: HDefs) ; self . start_typedef_struct (id) ; for (i , ty) in tuple . types . iter () . enumerate () { self . print_ty (SourceType :: HDefs , ty) ; uwriteln ! (self . src . h_defs , " f{i};") ; } self . finish_typedef_struct (id) ; } fn type_flags (& mut self , id : TypeId , name : & str , flags : & Flags , docs : & Docs) { self . src . h_defs ("\n") ; self . docs (docs , SourceType :: HDefs) ; self . src . h_defs ("typedef ") ; let repr = flags_repr (flags) ; self . src . h_defs (int_repr (repr)) ; self . src . h_defs (" ") ; self . print_typedef_target (id) ; if flags . flags . len () > 0 { self . src . h_defs ("\n") ; } let ns = self . owner_namespace (id) . to_shouty_snake_case () ; for (i , flag) in flags . flags . iter () . enumerate () { self . docs (& flag . docs , SourceType :: HDefs) ; uwriteln ! (self . src . h_defs , "#define {ns}_{}_{} (1 << {i})" , name . to_shouty_snake_case () , flag . name . to_shouty_snake_case () ,) ; } } fn type_variant (& mut self , id : TypeId , name : & str , variant : & Variant , docs : & Docs) { self . src . h_defs ("\n") ; self . docs (docs , SourceType :: HDefs) ; self . start_typedef_struct (id) ; self . src . h_defs (int_repr (variant . tag ())) ; self . src . h_defs (" tag;\n") ; let cases_with_data = Vec :: from_iter (variant . cases . iter () . filter_map (| case | case . ty . as_ref () . map (| ty | (& case . name , ty))) ,) ; if ! cases_with_data . is_empty () { self . src . h_defs ("union {\n") ; for (name , ty) in cases_with_data { self . print_ty (SourceType :: HDefs , ty) ; self . src . h_defs (" ") ; self . src . h_defs (& to_c_ident (name)) ; self . src . h_defs (";\n") ; } self . src . h_defs ("} val;\n") ; } self . finish_typedef_struct (id) ; if variant . cases . len () > 0 { self . src . h_defs ("\n") ; } let ns = self . owner_namespace (id) . to_shouty_snake_case () ; for (i , case) in variant . cases . iter () . enumerate () { self . docs (& case . docs , SourceType :: HDefs) ; uwriteln ! (self . src . h_defs , "#define {ns}_{}_{} {i}" , name . to_shouty_snake_case () , case . name . to_shouty_snake_case () ,) ; } } fn type_option (& mut self , id : TypeId , _name : & str , payload : & Type , docs : & Docs) { self . src . h_defs ("\n") ; self . docs (docs , SourceType :: HDefs) ; self . start_typedef_struct (id) ; self . src . h_defs ("bool is_some;\n") ; self . print_ty (SourceType :: HDefs , payload) ; self . src . h_defs (" val;\n") ; self . finish_typedef_struct (id) ; } fn type_result (& mut self , id : TypeId , _name : & str , result : & Result_ , docs : & Docs) { self . src . h_defs ("\n") ; self . docs (docs , SourceType :: HDefs) ; self . start_typedef_struct (id) ; self . src . h_defs ("bool is_err;\n") ; if result . ok . is_some () || result . err . is_some () { self . src . h_defs ("union {\n") ; if let Some (ok) = result . ok . as_ref () { self . print_ty (SourceType :: HDefs , ok) ; self . src . h_defs (" ok;\n") ; } if let Some (err) = result . err . as_ref () { self . print_ty (SourceType :: HDefs , err) ; self . src . h_defs (" err;\n") ; } self . src . h_defs ("} val;\n") ; } self . finish_typedef_struct (id) ; } fn type_enum (& mut self , id : TypeId , name : & str , enum_ : & Enum , docs : & Docs) { uwrite ! (self . src . h_defs , "\n") ; self . docs (docs , SourceType :: HDefs) ; let int_t = int_repr (enum_ . tag ()) ; uwrite ! (self . src . h_defs , "typedef {int_t} ") ; self . print_typedef_target (id) ; if enum_ . cases . len () > 0 { self . src . h_defs ("\n") ; } let ns = self . owner_namespace (id) . to_shouty_snake_case () ; for (i , case) in enum_ . cases . iter () . enumerate () { self . docs (& case . docs , SourceType :: HDefs) ; uwriteln ! (self . src . h_defs , "#define {ns}_{}_{} {i}" , name . to_shouty_snake_case () , case . name . to_shouty_snake_case () ,) ; } } fn type_alias (& mut self , id : TypeId , _name : & str , ty : & Type , docs : & Docs) { let target = dealias (self . resolve , id) ; if matches ! (& self . resolve . types [target] . kind , TypeDefKind :: Resource) { return ; } self . src . h_defs ("\n") ; self . docs (docs , SourceType :: HDefs) ; self . src . h_defs ("typedef ") ; self . print_ty (SourceType :: HDefs , ty) ; self . src . h_defs (" ") ; self . print_typedef_target (id) ; } fn type_list (& mut self , id : TypeId , _name : & str , ty : & Type , docs : & Docs) { self . src . h_defs ("\n") ; self . docs (docs , SourceType :: HDefs) ; self . start_typedef_struct (id) ; self . print_ty (SourceType :: HDefs , ty) ; self . src . h_defs (" *ptr;\n") ; self . src . h_defs ("size_t len;\n") ; self . finish_typedef_struct (id) ; } fn type_future (& mut self , id : TypeId , _name : & str , _ty : & Option < Type > , docs : & Docs) { self . src . h_defs ("\n") ; self . docs (docs , SourceType :: HDefs) ; self . src . h_defs ("\ntypedef uint32_t ") ; self . print_typedef_target (id) ; } fn type_stream (& mut self , id : TypeId , _name : & str , _ty : & Option < Type > , docs : & Docs) { self . src . h_defs ("\n") ; self . docs (docs , SourceType :: HDefs) ; self . src . h_defs ("\ntypedef uint32_t ") ; self . print_typedef_target (id) ; } fn type_builtin (& mut self , id : TypeId , name : & str , ty : & Type , docs : & Docs) { let _ = (id , name , ty , docs) ; } }
};
}
