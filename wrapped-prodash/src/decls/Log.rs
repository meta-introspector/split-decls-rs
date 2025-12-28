macro_rules! deps {
    () => {
        Id!();
        Unit!();
        NestedProgress!();
        StepShared!();
    };
}

macro_rules! Log {
    () => {
        deps!();
        # [doc = " A [`NestedProgress`] implementation which displays progress as it happens without the use of a renderer."] # [doc = ""] # [doc = " Note that this incurs considerable performance cost as each progress calls ends up getting the system time"] # [doc = " to see if progress information should actually be emitted."] pub struct Log { name : String , id : Id , max : Option < usize > , unit : Option < Unit > , step : StepShared , current_level : usize , max_level : usize , trigger : Arc < AtomicBool > , }
    };
}

Log!()