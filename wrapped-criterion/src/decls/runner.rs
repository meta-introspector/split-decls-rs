macro_rules! deps {
    () => {
        Criterion!();
    };
}

macro_rules! runner {
    () => {
        deps!();
        # [doc = " Custom-test-framework runner. Should not be called directly."] # [doc (hidden)] pub fn runner (benches : & [& dyn Fn ()]) { for bench in benches { bench () ; } Criterion :: default () . configure_from_args () . final_summary () ; }
    };
}

runner!()