macro_rules! deps {
    () => {
        TagForeachCB!();
    };
}

macro_rules! TagForeachData {
    () => {
        deps!();
        # [doc = " helper type to be able to pass callback to payload"] pub (crate) struct TagForeachData < 'a > { # [doc = " callback"] pub (crate) cb : TagForeachCB < 'a > , }
    };
}

TagForeachData!()