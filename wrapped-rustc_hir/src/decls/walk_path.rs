macro_rules! deps {
    () => {
        Visitor!();
        Path!();
    };
}

macro_rules! walk_path {
    () => {
        deps!();
        pub fn walk_path < 'v , V : Visitor < 'v > > (visitor : & mut V , path : & Path < 'v >) -> V :: Result { let Path { segments , span : _ , res : _ } = path ; walk_list ! (visitor , visit_path_segment , * segments) ; V :: Result :: output () }
    };
}

walk_path!();