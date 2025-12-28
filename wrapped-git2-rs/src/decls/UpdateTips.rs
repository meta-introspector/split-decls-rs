macro_rules! deps {
    () => {
        Oid!();
    };
}

macro_rules! UpdateTips {
    () => {
        deps!();
        # [doc = " Callback for whenever a reference is updated locally."] pub type UpdateTips < 'a > = dyn FnMut (& str , Oid , Oid) -> bool + 'a ;
    };
}

UpdateTips!()