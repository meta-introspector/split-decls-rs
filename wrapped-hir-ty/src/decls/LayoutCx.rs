macro_rules! LayoutCx {
    () => {
        struct LayoutCx < 'a > { calc : LayoutCalculator < & 'a TargetDataLayout > , }
    };
}

LayoutCx!()