macro_rules! FrameNote {
    () => {
        # [derive (Clone)] pub struct FrameNote { pub span : Span , pub times : i32 , pub where_ : & 'static str , pub instance : String , pub has_label : bool , }
    };
}

FrameNote!();