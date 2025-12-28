macro_rules! is_lit_name_ref {
    () => {
        # [doc = " Checks if a name reference is used in a literal (constructor) context."] # [doc = " Used to filter references when searching for struct/variant constructors."] # [doc = ""] # [doc = " # Returns"] # [doc = " `true` if the name reference is used as part of a struct/variant literal expression."] fn is_lit_name_ref (name_ref : & ast :: NameRef) -> bool { name_ref . syntax () . ancestors () . find_map (| ancestor | { match_ast ! { match ancestor { ast :: PathExpr (path_expr) => Some (path_ends_with (path_expr . path () , name_ref)) , ast :: RecordExpr (record_expr) => Some (path_ends_with (record_expr . path () , name_ref)) , _ => None , } } }) . unwrap_or (false) }
    };
}

is_lit_name_ref!();