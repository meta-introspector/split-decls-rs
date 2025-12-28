macro_rules! deps {
    () => {
        State!();
        Value!();
    };
}

macro_rules! progress_style {
    () => {
        deps!();
        fn progress_style (p : & Value) -> Style { use crate :: progress :: State :: * ; match p . state { Running => if let Some (fraction) = p . fraction () { if fraction > 0.8 { Color :: Green } else { Color :: Yellow } } else { Color :: White } . normal () , Halted (_ , _) => Color :: Red . dimmed () , Blocked (_ , _) => Color :: Red . normal () , } }
    };
}

progress_style!()