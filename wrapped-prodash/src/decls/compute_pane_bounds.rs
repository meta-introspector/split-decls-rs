macro_rules! deps {
    () => {
        Message!();
    };
}

macro_rules! compute_pane_bounds {
    () => {
        deps!();
        fn compute_pane_bounds (messages : & [Message] , inner : Rect , messages_fullscreen : bool) -> (Rect , Option < Rect >) { if messages . is_empty () { (inner , None) } else { let (task_percent , messages_percent) = if messages_fullscreen { (0.1 , 0.9) } else { (0.75 , 0.25) } ; let tasks_height : u16 = (inner . height as f32 * task_percent) . ceil () as u16 ; let messages_height : u16 = (inner . height as f32 * messages_percent) . floor () as u16 ; if messages_height < 2 { (inner , None) } else { let messages_title = 1u16 ; let new_messages_height = messages_height . min ((messages . len () + messages_title as usize) as u16) ; let tasks_height = tasks_height . saturating_add (messages_height - new_messages_height) ; let messages_height = new_messages_height ; (Rect { height : tasks_height , .. inner } , Some (rect :: intersect (Rect { y : tasks_height + messages_title , height : messages_height , .. inner } , inner ,)) ,) } } }
    };
}

compute_pane_bounds!()