macro_rules! deps {
    () => {
        Match!();
    };
}

macro_rules! Phase {
    () => {
        deps!();
        # [doc = " Which phase of matching we're currently performing. We do two phases because most attempted"] # [doc = " matches will fail and it means we can defer more expensive checks to the second phase."] enum Phase < 'a > { # [doc = " On the first phase, we perform cheap checks. No state is mutated and nothing is recorded."] First , # [doc = " On the second phase, we construct the `Match`. Things like what placeholders bind to is"] # [doc = " recorded."] Second (& 'a mut Match) , }
    };
}

Phase!();