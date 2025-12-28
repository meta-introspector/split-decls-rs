macro_rules! DisplayLifetime {
    () => {
        # [derive (Copy , Clone)] pub enum DisplayLifetime { Always , OnlyStatic , OnlyNamed , OnlyNamedOrStatic , Never , }
    };
}

DisplayLifetime!()