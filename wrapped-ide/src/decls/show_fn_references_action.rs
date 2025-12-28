macro_rules! deps {
    () => {
        HoverAction!();
        UpmappingResult!();
    };
}

macro_rules! show_fn_references_action {
    () => {
        deps!();
        fn show_fn_references_action (sema : & Semantics < '_ , RootDatabase > , def : Definition ,) -> Option < HoverAction > { match def { Definition :: Function (it) => { it . try_to_nav (sema) . map (UpmappingResult :: call_site) . map (| nav_target | { HoverAction :: Reference (FilePosition { file_id : nav_target . file_id , offset : nav_target . focus_or_full_range () . start () , }) }) } _ => None , } }
    };
}

show_fn_references_action!();