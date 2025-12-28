macro_rules! deps {
    () => {
        Stmt!();
    };
}

macro_rules! StmtList {
    () => {
        deps!();
        # [doc = " A list of statements. This corresponds to the `stmt_list` non-terminal of the"] # [doc = " grammar."] # [derive (Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct StmtList < A > { # [doc = " The list of statements."] pub stmts : Vec < Stmt < A > > , }
    };
}

StmtList!();