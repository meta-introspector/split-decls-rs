macro_rules! deps {
    () => {
        Printer!();
        FixupContext!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Printer { pub fn stmt (& mut self , stmt : & Stmt , is_last : bool) { match stmt { Stmt :: Local (local) => { self . outer_attrs (& local . attrs) ; self . ibox (0) ; self . word ("let ") ; self . pat (& local . pat) ; if let Some (local_init) = & local . init { self . word (" = ") ; self . neverbreak () ; self . subexpr (& local_init . expr , local_init . diverge . is_some () && classify :: expr_trailing_brace (& local_init . expr) , FixupContext :: NONE ,) ; if let Some ((_else , diverge)) = & local_init . diverge { self . space () ; self . word ("else ") ; self . end () ; self . neverbreak () ; self . cbox (INDENT) ; if let Some (expr) = expr :: simple_block (diverge) { self . small_block (& expr . block , & []) ; } else { self . expr_as_small_block (diverge , INDENT) ; } } } self . end () ; self . word (";") ; self . hardbreak () ; } Stmt :: Item (item) => self . item (item) , Stmt :: Expr (expr , None) => { if break_after (expr) { self . ibox (0) ; self . expr_beginning_of_line (expr , false , true , FixupContext :: new_stmt ()) ; if add_semi (expr) { self . word (";") ; } self . end () ; self . hardbreak () ; } else { self . expr_beginning_of_line (expr , false , true , FixupContext :: new_stmt ()) ; } } Stmt :: Expr (expr , Some (_semi)) => { if let Expr :: Verbatim (tokens) = expr { if tokens . is_empty () { return ; } } self . ibox (0) ; self . expr_beginning_of_line (expr , false , true , FixupContext :: new_stmt ()) ; if ! remove_semi (expr) { self . word (";") ; } self . end () ; self . hardbreak () ; } Stmt :: Macro (stmt) => { self . outer_attrs (& stmt . attrs) ; let semicolon = stmt . semi_token . is_some () || ! is_last && mac :: requires_semi (& stmt . mac . delimiter) ; self . mac (& stmt . mac , None , semicolon) ; self . hardbreak () ; } } } }
    };
}

impl_79!()