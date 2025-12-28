macro_rules! deps {
    () => {
        MacroUseArgs!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Default for MacroUseArgs { fn default () -> Self { Self :: UseSpecific (ThinVec :: new ()) } }
    };
}

impl_17!();