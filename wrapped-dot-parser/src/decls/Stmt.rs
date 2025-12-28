macro_rules! deps {
    () => {
        AttrStmt!();
        EdgeStmt!();
        IDEq!();
        NodeStmt!();
    };
}

macro_rules! Stmt {
    () => {
        deps!();
        # [doc = " A statement of the graph. This corresponds to the `stmt` non-terminal of the"] # [doc = " grammar."] # [derive (Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum Stmt < A > { # [doc = " A node statement."] NodeStmt (NodeStmt < A >) , # [doc = " An edge statement."] EdgeStmt (EdgeStmt < A >) , # [doc = " An attribute statement."] AttrStmt (AttrStmt < A >) , # [doc = " An alias statement."] IDEq (String , String) , }
    };
}

Stmt!()