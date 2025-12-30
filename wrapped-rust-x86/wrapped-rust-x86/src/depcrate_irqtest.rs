// Generated macro for test (module)
macro_rules! Depcrate_irqtest {
() => {
// Module: crate::irq
// Provides: {"test"}
// Dependencies: {}
# [cfg (all (test , feature = "utest"))] mod test { use super :: * ; # [test] fn bit_macro () { assert ! (PageFaultError :: PK . bits () == 0b100000) ; assert ! (PageFaultError :: ID . bits () == 0b10000) ; assert ! (PageFaultError :: RSVD . bits () == 0b1000) ; assert ! (PageFaultError :: US . bits () == 0b100) ; assert ! (PageFaultError :: WR . bits () == 0b10) ; assert ! (PageFaultError :: P . bits () == 0b1) ; } }
};
}
