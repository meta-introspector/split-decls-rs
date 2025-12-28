macro_rules! deps {
    () => {
        TypeCollector!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < 'ast , 'a > Visit < 'ast > for TypeCollector < 'a > { fn visit_type (& mut self , i : & 'ast Type) { let type_str = i . to_token_stream () . to_string () ; let primitive_types = ["bool" , "char" , "f32" , "f64" , "i8" , "i16" , "i32" , "i64" , "i128" , "isize" , "u8" , "u16" , "u32" , "u64" , "u128" , "usize" , "str" , "String" , "Vec" , "Option" , "Result" , "HashMap" , "HashSet" , "Box" , "Arc" , "Rc" ,] ; if ! primitive_types . contains (& type_str . as_str ()) { match i { Type :: Path (type_path) => { if let Some (segment) = type_path . path . segments . last () { self . types . insert (segment . ident . to_string ()) ; } } _ => { self . types . insert (type_str) ; } } } visit :: visit_type (self , i) ; } }
    };
}

impl_180!()