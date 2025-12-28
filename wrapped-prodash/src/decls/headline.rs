macro_rules! deps {
    () => {
        Duration!();
        Key!();
        State!();
        Task!();
        InterruptDrawInfo!();
    };
}

macro_rules! headline {
    () => {
        deps!();
        pub (crate) fn headline (entries : & [(Key , Task)] , interrupt_mode : InterruptDrawInfo , duration_per_frame : Duration , buf : & mut Buffer , bound : Rect ,) { let (num_running_tasks , num_blocked_tasks , num_groups) = entries . iter () . fold ((0 , 0 , 0) , | (mut running , mut blocked , mut groups) , (_key , Task { progress , .. }) | { match progress . as_ref () . map (| p | p . state) { Some (progress :: State :: Running) => running += 1 , Some (progress :: State :: Blocked (_ , _)) | Some (progress :: State :: Halted (_ , _)) => blocked += 1 , None => groups += 1 , } (running , blocked , groups) } ,) ; let text = format ! (" {} {} {:3} running + {:3} blocked + {:3} groups = {} " , match interrupt_mode { InterruptDrawInfo :: Instantly => "'q' or CTRL+c to quit" , InterruptDrawInfo :: Deferred (interrupt_requested) => { if interrupt_requested { "interrupt requested - please wait" } else { "cannot interrupt current operation" } } } , if duration_per_frame > Duration :: from_secs (1) { format ! (" Every {}s → {}" , duration_per_frame . as_secs () , format_now_datetime_seconds ()) } else { "" . into () } , num_running_tasks , num_blocked_tasks , num_groups , entries . len ()) ; let bold = Style :: default () . add_modifier (Modifier :: BOLD) ; draw_text_with_ellipsis_nowrap (rect :: snap_to_right (bound , block_width (& text) + 1) , buf , text , bold) ; }
    };
}

headline!()