// Generated macro for IndependentStrategy (type)
macro_rules! Depcrate_strategyIndependentStrategy {
() => {
// Module: crate::strategy
// Provides: {"IndependentStrategy"}
// Dependencies: {}
# [doc = " Strategy for isolating instances."] # [doc = ""] # [doc = " It is similar to [`DefaultStrategy`], however the spin lock is not sharded (therefore multiple"] # [doc = " concurrent threads might get bigger hit when multiple threads have to fall back). Nevertheless,"] # [doc = " each instance has a private spin lock, not influencing the other instances. That also makes"] # [doc = " them bigger in memory."] # [doc = ""] # [doc = " The hazard pointers are still shared between all instances."] # [doc = ""] # [doc = " The purpose of this strategy is meant for cases where a single instance is going to be"] # [doc = " \"tortured\" a lot, so it should not overflow to other instances."] # [doc = ""] # [doc = " This too may be changed for something else (but with at least as good guarantees, primarily"] # [doc = " that other instances won't get influenced by the \"torture\")."] # [doc (hidden)] pub type IndependentStrategy = DefaultStrategy ;
};
}
