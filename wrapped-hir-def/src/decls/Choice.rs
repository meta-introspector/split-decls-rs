macro_rules! deps {
    () => {
        Stability!();
    };
}

macro_rules! Choice {
    () => {
        deps!();
        # [derive (Debug)] struct Choice { path : ModPath , # [doc = " The length in characters of the path"] path_text_len : usize , # [doc = " The stability of the path"] stability : Stability , # [doc = " Whether this path contains a prelude segment and preference for it has been signaled"] prefer_due_to_prelude : bool , }
    };
}

Choice!()