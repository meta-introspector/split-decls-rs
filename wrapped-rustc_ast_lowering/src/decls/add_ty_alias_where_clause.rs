macro_rules! add_ty_alias_where_clause {
    () => {
        # [doc = " When we have a ty alias we *may* have two where clauses. To give the best diagnostics, we set the span"] # [doc = " to the where clause that is preferred, if it exists. Otherwise, it sets the span to the other where"] # [doc = " clause if it exists."] fn add_ty_alias_where_clause (generics : & mut ast :: Generics , mut where_clauses : TyAliasWhereClauses , prefer_first : bool ,) { if ! prefer_first { (where_clauses . before , where_clauses . after) = (where_clauses . after , where_clauses . before) ; } let where_clause = if where_clauses . before . has_where_token || ! where_clauses . after . has_where_token { where_clauses . before } else { where_clauses . after } ; generics . where_clause . has_where_token = where_clause . has_where_token ; generics . where_clause . span = where_clause . span ; }
    };
}

add_ty_alias_where_clause!()