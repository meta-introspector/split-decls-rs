macro_rules! Output {
    () => {
        # [doc = " Output file path"] pub struct Output (Cow < 'static , Path >) ;
    };
}

Output!()