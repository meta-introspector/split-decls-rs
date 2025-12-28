macro_rules! deps {
    () => {
        AutoLogParse!();
        IndexStr!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl AutoLogParse { # [cfg (feature = "logging")] fn new (production : & 'static str , input : IndexStr < '_ >) -> AutoLogParse { LOG_DEPTH . with (| depth | { if * depth . borrow () == 0 { println ! () ; } let indent : String = (0 .. * depth . borrow () * 4) . map (| _ | ' ') . collect () ; log ! ("{}({} \"{}\" {}" , indent , production , String :: from_utf8_lossy (input . as_ref ()) , input . len () ,) ; * depth . borrow_mut () += 1 ; }) ; AutoLogParse } # [cfg (not (feature = "logging"))] # [inline (always)] fn new (_ : & 'static str , _ : IndexStr) -> AutoLogParse { AutoLogParse } }
    };
}

impl_8!()