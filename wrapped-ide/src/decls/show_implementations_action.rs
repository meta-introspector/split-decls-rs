macro_rules! deps {
    () => {
        NavigationTarget!();
        UpmappingResult!();
        HoverAction!();
    };
}

macro_rules! show_implementations_action {
    () => {
        deps!();
        fn show_implementations_action (sema : & Semantics < '_ , RootDatabase > , def : Definition ,) -> Option < HoverAction > { fn to_action (nav_target : NavigationTarget) -> HoverAction { HoverAction :: Implementation (FilePosition { file_id : nav_target . file_id , offset : nav_target . focus_or_full_range () . start () , }) } let adt = match def { Definition :: Trait (it) => { return it . try_to_nav (sema) . map (UpmappingResult :: call_site) . map (to_action) ; } Definition :: Adt (it) => Some (it) , Definition :: SelfType (it) => it . self_ty (sema . db) . as_adt () , _ => None , } ? ; adt . try_to_nav (sema) . map (UpmappingResult :: call_site) . map (to_action) }
    };
}

show_implementations_action!()