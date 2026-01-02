mkuse!{use crate :: expression :: LetVariant ;}
mkuse!{use crate :: wildstring :: WildStringPart ;}
mkuse!{use crate :: { expression :: { Expression , IdentifierType } , typekinds :: * , wildstring :: WildString , } ;}

macro_rules! create_single_wild_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_single_wild_string in module {}", module_path!());
    };
}

mkfn!{
    create_single_wild_string_introspect!();
    # [doc = " Simplifies creating a string that can be used in an Expression, as Expression"] # [doc = " expects all strings to be `WildString`"] fn create_single_wild_string (name : & str) -> WildString { WildString (vec ! [WildStringPart :: String (name . to_string ())]) }
}

macro_rules! create_symbol_identifier_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_symbol_identifier in module {}", module_path!());
    };
}

mkfn!{
    create_symbol_identifier_introspect!();
    # [doc = " Creates an Identifier with name `name` with no wildcards. This, for example,"] # [doc = " can be used to create variables, function names or arbitrary input. Is is"] # [doc = " extremely flexible."] pub fn create_symbol_identifier (arbitrary_string : & str) -> Expression { let identifier_name = create_single_wild_string (arbitrary_string) ; Expression :: Identifier (identifier_name , IdentifierType :: Symbol) }
}

macro_rules! create_array_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_array in module {}", module_path!());
    };
}

mkfn!{
    create_array_introspect!();
    # [doc = " To compose the simd_shuffle! call we need:"] # [doc = " - simd_shuffle!(<arg1>, <arg2>, <array>)"] # [doc = ""] # [doc = " Here we are creating a string version of the `<array>` that can be used as an"] # [doc = " Expression Identifier"] # [doc = ""] # [doc = " In textual form `a: int32x4_t` which has 4 lanes would generate:"] # [doc = " ```"] # [doc = " [0, 1, 2, 3]"] # [doc = " ```"] fn create_array (lanes : u32) -> Option < String > { match lanes { 1 => None , 2 => Some ("[1, 0]" . to_string ()) , 3 => Some ("[2, 1, 0]" . to_string ()) , 4 => Some ("[3, 2, 1, 0]" . to_string ()) , 8 => Some ("[7, 6, 5, 4, 3, 2, 1, 0]" . to_string ()) , 16 => Some ("[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]" . to_string ()) , _ => panic ! ("Incorrect vector number of vector lanes: {lanes}") , } }
}

macro_rules! create_let_variable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_let_variable in module {}", module_path!());
    };
}

mkfn!{
    create_let_variable_introspect!();
    # [doc = " Creates: `let <variable_name>: <type> = <expression>`"] pub fn create_let_variable (variable_name : & str , type_kind : & TypeKind , expression : Expression ,) -> Expression { let identifier_name = create_single_wild_string (variable_name) ; Expression :: Let (LetVariant :: WithType (identifier_name , type_kind . clone () , Box :: new (expression) ,)) }
}

macro_rules! create_mut_let_variable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_mut_let_variable in module {}", module_path!());
    };
}

mkfn!{
    create_mut_let_variable_introspect!();
    pub fn create_mut_let_variable (variable_name : & str , type_kind : & TypeKind , expression : Expression ,) -> Expression { let identifier_name = create_single_wild_string (variable_name) ; Expression :: Let (LetVariant :: MutWithType (identifier_name , type_kind . clone () , Box :: new (expression) ,)) }
}

macro_rules! type_has_tuple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_has_tuple in module {}", module_path!());
    };
}

mkfn!{
    type_has_tuple_introspect!();
    pub fn type_has_tuple (type_kind : & TypeKind) -> bool { if let TypeKind :: Vector (vector_type) = type_kind { vector_type . tuple_size () . is_some () } else { false } }
}

macro_rules! make_variable_mutable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_variable_mutable in module {}", module_path!());
    };
}

mkfn!{
    make_variable_mutable_introspect!();
    pub fn make_variable_mutable (variable_name : & str , type_kind : & TypeKind) -> Expression { let mut_variable = format ! ("let mut {variable_name}: {type_kind} = {variable_name}") ; let identifier_name = create_single_wild_string (& mut_variable) ; Expression :: Identifier (identifier_name , IdentifierType :: Symbol) }
}

macro_rules! create_shuffle_internal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_shuffle_internal in module {}", module_path!());
    };
}

mkfn!{
    create_shuffle_internal_introspect!();
    # [doc = " For creating shuffle calls, accepts function pointers for formatting for tuple"] # [doc = " types and types without a tuple"] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " `a: int32x4_t` with formatting function `create_shuffle_call_fmt` creates:"] # [doc = " ```"] # [doc = " simd_shuffle!(a, a, [0, 1, 2, 3])"] # [doc = " ```"] # [doc = ""] # [doc = " `a: int32x4x2_t` creates:"] # [doc = " ```"] # [doc = " a.0 = simd_shuffle!(a.0, a.0, [0, 1, 2, 3])"] # [doc = " a.1 = simd_shuffle!(a.1, a.1, [0, 1, 2, 3])"] # [doc = " ```"] fn create_shuffle_internal (variable_name : & String , type_kind : & TypeKind , fmt_tuple : fn (variable_name : & String , idx : u32 , array_lanes : & String) -> String , fmt : fn (variable_name : & String , type_kind : & TypeKind , array_lanes : & String) -> String ,) -> Option < Expression > { let TypeKind :: Vector (vector_type) = type_kind else { return None ; } ; let lane_count = vector_type . lanes () ; let array_lanes = create_array (lane_count) ? ; let tuple_count = vector_type . tuple_size () . map_or_else (| | 0 , | t | t . to_int ()) ; if tuple_count > 0 { let capacity_estimate : usize = tuple_count as usize * (lane_count as usize + ((variable_name . len () + 2) * 3)) ; let mut string_builder = String :: with_capacity (capacity_estimate) ; for idx in 0 .. tuple_count { let formatted = fmt_tuple (variable_name , idx , & array_lanes) ; string_builder += formatted . as_str () ; } Some (create_symbol_identifier (& string_builder)) } else { let expression = fmt (variable_name , type_kind , & array_lanes) ; Some (create_symbol_identifier (& expression)) } }
}

macro_rules! create_assigned_tuple_shuffle_call_fmt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_assigned_tuple_shuffle_call_fmt in module {}", module_path!());
    };
}

mkfn!{
    create_assigned_tuple_shuffle_call_fmt_introspect!();
    fn create_assigned_tuple_shuffle_call_fmt (variable_name : & String , idx : u32 , array_lanes : & String ,) -> String { format ! ("{variable_name}.{idx} = unsafe {{ simd_shuffle!({variable_name}.{idx}, {variable_name}.{idx}, {array_lanes}) }};\n") }
}

macro_rules! create_assigned_shuffle_call_fmt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_assigned_shuffle_call_fmt in module {}", module_path!());
    };
}

mkfn!{
    create_assigned_shuffle_call_fmt_introspect!();
    fn create_assigned_shuffle_call_fmt (variable_name : & String , type_kind : & TypeKind , array_lanes : & String ,) -> String { format ! ("let {variable_name}: {type_kind} = unsafe {{ simd_shuffle!({variable_name}, {variable_name}, {array_lanes}) }}") }
}

macro_rules! create_shuffle_call_fmt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_shuffle_call_fmt in module {}", module_path!());
    };
}

mkfn!{
    create_shuffle_call_fmt_introspect!();
    fn create_shuffle_call_fmt (variable_name : & String , _type_kind : & TypeKind , array_lanes : & String ,) -> String { format ! ("simd_shuffle!({variable_name}, {variable_name}, {array_lanes})") }
}

macro_rules! create_assigned_shuffle_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_assigned_shuffle_call in module {}", module_path!());
    };
}

mkfn!{
    create_assigned_shuffle_call_introspect!();
    # [doc = " Create a `simd_shuffle!(<...>, [...])` call, where the output is stored"] # [doc = " in a variable named `variable_name`"] pub fn create_assigned_shuffle_call (variable_name : & String , type_kind : & TypeKind ,) -> Option < Expression > { create_shuffle_internal (variable_name , type_kind , create_assigned_tuple_shuffle_call_fmt , create_assigned_shuffle_call_fmt ,) }
}

macro_rules! create_shuffle_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_shuffle_call in module {}", module_path!());
    };
}

mkfn!{
    create_shuffle_call_introspect!();
    # [doc = " Create a `simd_shuffle!(<...>, [...])` call"] pub fn create_shuffle_call (variable_name : & String , type_kind : & TypeKind) -> Option < Expression > { create_shuffle_internal (variable_name , type_kind , create_assigned_tuple_shuffle_call_fmt , create_shuffle_call_fmt ,) }
}