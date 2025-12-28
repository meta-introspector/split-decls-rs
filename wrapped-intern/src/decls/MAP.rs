macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! MAP {
    () => {
        deps!();
        static MAP : OnceLock < Map > = OnceLock :: new () ;
    };
}

MAP!()