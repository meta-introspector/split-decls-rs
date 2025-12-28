macro_rules! deps {
    () => {
        JoinAllKind!();
    };
}

macro_rules! JoinAll {
    () => {
        deps!();
        # [must_use = "futures do nothing unless you `.await` or poll them"] # [doc = " Future for the [`join_all`] function."] pub struct JoinAll < F > where F : Future , { kind : JoinAllKind < F > , }
    };
}

JoinAll!()