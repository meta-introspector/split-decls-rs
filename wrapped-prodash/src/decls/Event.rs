macro_rules! Event {
    () => {
        # [derive (Debug)] enum Event { Tick , Quit , # [cfg (feature = "signal-hook")] Resize (u16 , u16) , }
    };
}

Event!()