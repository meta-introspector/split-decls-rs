macro_rules! deps {
    () => {
        Definition!();
        RootDatabase!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl ReferenceCategory { fn new (sema : & Semantics < '_ , RootDatabase > , def : & Definition , r : & ast :: NameRef ,) -> ReferenceCategory { let mut result = ReferenceCategory :: empty () ; if is_name_ref_in_test (sema , r) { result |= ReferenceCategory :: TEST ; } if ! matches ! (def , Definition :: Local (_) | Definition :: Field (_)) { if is_name_ref_in_import (r) { result |= ReferenceCategory :: IMPORT ; } return result ; } let mode = r . syntax () . ancestors () . find_map (| node | { match_ast ! { match node { ast :: BinExpr (expr) => { if matches ! (expr . op_kind () ?, ast :: BinaryOp :: Assignment { .. }) { if let Some (lhs) = expr . lhs () && lhs . syntax () . text_range () . end () == r . syntax () . text_range () . end () { return Some (ReferenceCategory :: WRITE) } } Some (ReferenceCategory :: READ) } , _ => None , } } }) . unwrap_or (ReferenceCategory :: READ) ; result | mode } }
    };
}

impl_171!()