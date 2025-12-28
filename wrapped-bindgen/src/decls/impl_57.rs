macro_rules! deps {
    () => {
        ReferenceStyle!();
        ReferenceStage!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl ReferenceStage { # [track_caller] pub fn parse (mut arg : & str) -> Self { if arg == "windows" { arg = "windows,skip-root,Windows" } let arg : Vec < _ > = arg . split (',') . collect () ; if arg . len () != 3 { invalid_reference () ; } Self { name : arg [0] . to_string () , style : ReferenceStyle :: parse (arg [1]) , path : arg [2] . to_string () , } } }
    };
}

impl_57!()