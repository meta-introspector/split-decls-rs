macro_rules! CompletedMarker {
    () => {
        pub (crate) struct CompletedMarker { start_pos : u32 , end_pos : u32 , kind : SyntaxKind , }
    };
}

CompletedMarker!();