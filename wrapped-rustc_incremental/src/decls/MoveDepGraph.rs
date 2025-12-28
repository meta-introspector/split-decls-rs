macro_rules! MoveDepGraph {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_move_dep_graph)] pub (crate) struct MoveDepGraph < 'a > { pub from : & 'a Path , pub to : & 'a Path , pub err : std :: io :: Error , }
    };
}

MoveDepGraph!()