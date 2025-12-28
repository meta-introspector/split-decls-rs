macro_rules! deps {
    () => {
        AutoLogParse!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        # [cfg (feature = "logging")] impl Drop for AutoLogParse { fn drop (& mut self) { LOG_DEPTH . with (| depth | { * depth . borrow_mut () -= 1 ; let indent : String = (0 .. * depth . borrow () * 4) . map (| _ | ' ') . collect () ; log ! ("{})" , indent) ; }) ; } }
    };
}

impl_9!()