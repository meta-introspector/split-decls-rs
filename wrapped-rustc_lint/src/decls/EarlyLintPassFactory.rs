macro_rules! deps {
    () => {
        EarlyLintPassObject!();
    };
}

macro_rules! EarlyLintPassFactory {
    () => {
        deps!();
        type EarlyLintPassFactory = dyn Fn () -> EarlyLintPassObject + sync :: DynSend + sync :: DynSync ;
    };
}

EarlyLintPassFactory!();